use std::collections::HashSet;

use async_trait::async_trait;
use chrono::{NaiveDateTime, TimeZone};
use chrono_tz::Asia::Seoul;
use fantoccini::{elements::Element, error::CmdError, Client, Locator};
use prost_types::Timestamp;
use serde_json::{json, Value};

use super::structs::{Card, CardTransaction};
use super::util::{parse_table, NodeExtension};
use super::webdriver;

#[async_trait]
trait WebDriverClientExtension {
    async fn click_keypad_at(&mut self, x: isize, y: isize) -> Result<Value, CmdError>;
}

#[async_trait]
impl WebDriverClientExtension for Client {
    async fn click_keypad_at(&mut self, x: isize, y: isize) -> Result<Value, CmdError> {
        self.execute(
            "
                const nodes = document.querySelectorAll('.askd_ui li.sp');
                for (const node of nodes) {
                    const style = window.getComputedStyle(node);
                    if (style.backgroundPositionX === arguments[0] + 'px' && style.backgroundPositionY === arguments[1] + 'px') {
                        const rect = node.getBoundingClientRect();
                        const event = new MouseEvent('click', {clientX: rect.x, clientY: rect.y, bubbles: true})
                        node.dispatchEvent(event);
                        return;
                    }
                }
                throw 'no such element';
            ",
            vec![json!(x), json!(y)],
        )
        .await
    }
}

#[async_trait]
trait WebDriverElementExtension {
    async fn click_in_js(&self) -> Result<(), CmdError>;
    async fn click_in_js_nowait(&self) -> Result<(), CmdError>;
}

#[async_trait]
impl WebDriverElementExtension for Element {
    async fn click_in_js(&self) -> Result<(), CmdError> {
        let mut c = self.clone().client();
        let current_url = c.current_url().await?;

        c.execute("arguments[0].click()", vec![json!(self)]).await?;
        c.wait_for_navigation(Some(current_url)).await?;
        c.wait_for(move |c| {
            let mut c = c.clone();
            async move { Ok(c.execute("return document.readyState;", vec![]).await.unwrap() == "complete") }
        })
        .await?;

        Ok(())
    }

    async fn click_in_js_nowait(&self) -> Result<(), CmdError> {
        let mut c = self.clone().client();

        c.execute("arguments[0].click()", vec![json!(self)]).await?;

        Ok(())
    }
}
pub async fn scrap_kbcard(id: &str, password: &str) -> Result<(Vec<Card>, Vec<CardTransaction>), CmdError> {
    if password.len() > 12 {
        panic!("kbcard password length must be lower or equal than 12 chars");
    }

    let mut c = webdriver::create_webdriver_client().await;

    c.goto("https://card.kbcard.com").await?;

    // To login page. We use js click because event banner might hide login button.
    c.find(Locator::Id("loginLinkBtn")).await?.click_in_js().await?;

    // Show login form
    c.find(Locator::Id("perTab01")).await?.click().await?;

    // Input credentials

    c.find(Locator::Id("인터넷서비스로그인ID")).await?.send_keys(&id).await?;
    c.find(Locator::Id("loginPwd")).await?.click().await?;

    c.wait_for_find(Locator::Css(".askd_ui")).await?;

    let keys = [
        vec!['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'],
        vec!['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'],
        vec!['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'z'],
        vec!['x', 'c', 'v', 'b', 'n', 'm', 'Q', 'W', 'E', 'R'],
        vec!['T', 'Y', 'U', 'I', 'O', 'P', 'A', 'S', 'D', 'F'],
        vec!['G', 'H', 'J', 'K', 'L', 'Z', 'X', 'C', 'V', 'B'],
        vec!['N', 'M', '`', '-', '=', '[', ']', '/', ';', '\''],
        vec![',', '.', '/', '~', '!', '@', '#', '$', '%', '^'],
        vec!['&', '*', '(', ')', '_', '+', '{', '}', '|', ':'],
        vec!['"', '<', '>', '?', ' '],
    ];
    let shift_specialchars = [
        '~', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+', '{', '}', '|', ':', '"', '<', '>', '?',
    ];

    let key_width = 42;
    let key_height = 34;
    let find_key_pos = |key| {
        for (y, row) in keys.iter().enumerate() {
            for (x, &item) in row.iter().enumerate() {
                if item == key {
                    return [-(x as isize) * key_width, -(y as isize) * key_height];
                }
            }
        }
        panic!()
    };

    for password_char in password.chars() {
        let [x, y] = find_key_pos(password_char);
        if password_char.is_uppercase() || shift_specialchars.contains(&password_char) {
            c.click_keypad_at(-180, -340).await?; // shift
        }
        c.click_keypad_at(x, y).await?;
    }
    c.click_keypad_at(-278, -374).await?; // finish

    c.find(Locator::Id("doIdLogin")).await?.click_in_js().await?;

    let title = c.execute("return document.title;", vec![]).await?;
    if title.as_str().unwrap().contains("멀티로그인(로그인전)") {
        c.find(Locator::Css(".kbBtn.btnL")).await?.click().await?;
    }

    // 카드이용·매출전표
    c.goto("https://card.kbcard.com/CXPRIMYS0007.cms").await?;

    // 일별조회
    c.find(Locator::Id("sample11")).await?.click_in_js_nowait().await?;

    c.find(Locator::Id("시작일자1")).await?.clear().await?;
    c.find(Locator::Id("시작일자1")).await?.send_keys("20200305").await?;
    c.find(Locator::Id("종료일자1")).await?.clear().await?;
    c.find(Locator::Id("종료일자1")).await?.send_keys("20210304").await?;

    c.find(Locator::Id("inquryTableBtn")).await?.click_in_js_nowait().await?;
    c.wait_for(|c| {
        let mut c = c.clone();
        async move {
            Ok(c.execute("return document.querySelectorAll('#popup_container').length;", vec![])
                .await
                .unwrap()
                == 0)
        }
    })
    .await?;

    let result = c.find(Locator::Css("#ajaxResultDiv .tblH")).await?.html(false).await?;

    c.find(Locator::Css(".kbBtn.btnS.logout")).await?.click_in_js().await?;

    c.close().await?;

    println!("{}", result);
    Ok(parse(&result))
}

// (amount, currency)
fn parse_amount_currency(amount_str: &str) -> (String, String) {
    // $99.99
    // 3,906원

    // TODO validation
    let (amount, currency) = amount_str.chars().partition(|x| x.is_numeric() || *x == '.');

    (amount, currency.replace(',', ""))
}

fn parse(table: &str) -> (Vec<Card>, Vec<CardTransaction>) {
    let mut cards = HashSet::new();
    let mut transactions = Vec::new();

    let table = parse_table(table).unwrap();
    for item in table {
        /*
           <div class="popBalloon">
               <span>CardIdent</span>
               <p>CardCardCard</p>
           </div>
           <input type="hidden" id="마스킹카드번호" name="마스킹카드번호" value="0000-00**-****-0000">
        */
        let card_name = item["이용 카드명"].maybe_element("td").unwrap().children[0]
            .maybe_element("div")
            .unwrap()
            .children[1]
            .inner_text();
        let card_no_input = item["이용 카드명"].maybe_element("td").unwrap().children[1]
            .maybe_element("input")
            .unwrap();
        let card_no = card_no_input.attributes["value"].as_ref().unwrap();
        let card = Card {
            display_name: card_name,
            card_no: card_no.into(),
        };

        cards.insert(card);

        let date = Seoul
            .from_local_datetime(&NaiveDateTime::parse_from_str(&item["이용일시"].inner_text(), "%Y.%m.%d %H:%M").unwrap())
            .unwrap();

        let (amount, currency) = parse_amount_currency(&item["이용금액"].inner_text());

        let raw_month = item["결제방법"].inner_text();
        let month = if raw_month == "일시불" {
            0
        } else {
            raw_month.parse::<i32>().unwrap()
        };

        // <a href="#none" class="linkPoint2 letterType1" title="Amazon_AWS" onclick="popSlsl('merchant id1' , 'base64 value','1', 'pg id1', '', '', '', '')">Amazon_AWS</a>
        // <a href="#none" class="linkPoint2 letterType1" title="GOOGLE* Domains" onclick="EngPopSlsl('base64 value','4')">GOOGLE* Domains</a>
        let merchant_onclick = &item["이용하신곳"].maybe_element("td").unwrap().children[0]
            .maybe_element("a")
            .unwrap()
            .attributes["onclick"];
        let merchant_id = if currency == "원" {
            Some(merchant_onclick.as_ref().unwrap().split('\'').nth(1).unwrap().into())
        } else {
            None
        };

        let canceled = match item["상태"].inner_text().as_str() {
            "전표매입" => false,
            "취소전표매입" => true,
            _ => panic!("Unknown status {}", item["상태"].inner_text()),
        };

        let transaction = CardTransaction {
            card_no: card_no.into(),
            transaction_id: item["승인번호"].inner_text(),
            date: Some(Timestamp {
                seconds: date.timestamp(),
                nanos: 0,
            }),
            amount,
            currency,
            merchant_id,
            merchant: item["이용하신곳"].inner_text(),
            month,
            canceled,
        };

        transactions.push(transaction);
    }

    (cards.into_iter().collect::<Vec<_>>(), transactions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[async_std::test]
    #[cfg(feature = "test_local")]
    async fn kbcard_test() -> Result<(), Box<dyn std::error::Error>> {
        dotenv::dotenv().ok();

        let id = std::env::var("KBCARD_ID")?;
        let password = std::env::var("KBCARD_PW")?;

        scrap_kbcard(&id, &password).await?;

        Ok(())
    }

    #[test]
    fn kbcard_parse_test() {
        let table = r###"
<table>
    <caption>전체카드 상세이용내역</caption>
        <colgroup>
            <col width="10%">
            <col width="7%">
            <col width="*">
            <col width="12%">
            <col width="9%" span="3">
            <col width="9%">
            <col width="10%">   
            <col width="10%">   
            <col width="9%">    
        </colgroup>
        <thead>      
        <tr>           
            <th scope="col">이용일시</th>     
            <th scope="col">이용<br>카드명</th>
            <th scope="col">이용하신곳</th>   
            <th scope="col">이용금액</th>
            <th scope="col">결제방법</th>       
            <th scope="col">가맹점<br>정보</th>
            <th scope="col">할인금액</th>
            <th scope="col">적립예상<br>포인트리</th>
            <th scope="col">상태</th>
            <th scope="col">결제예정일</th>
            <th scope="col">승인번호</th>   
        </tr>
        </thead>
        <tbody>

           
       <tr>
       <td>
           2021.03.03<br>23:59
           </td><td>
                <div class="popBalloon">
                    <span>CardIdent</span>
                    <p>CardCardCard</p>
                </div>
                <input type="hidden" id="마스킹카드번호" name="마스킹카드번호" value="0000-00**-****-0000">
           </td><td class="t_left"><a href="#none" class="linkPoint2 letterType1" title="Amazon_AWS" onclick="popSlsl('merchant id1' , 'base64 value','1', 'pg id1', '', '', '', '')">Amazon_AWS</a>
           </td><td>3,906원
           </td><td>일시불   
           </td><td>      
           </td><td>0
           </td><td class="t_right">0 P
           </td><td> 전표매입
           </td><td>2021.04.01    
           </td>     
           <td>1</td>     
       </tr>
   
       <tr>
       <td>
           2021.03.02<br>23:59
           </td><td>
                <div class="popBalloon">
                    <span>CardIdent</span>
                    <p>CardCardCard</p>
                </div>
                <input type="hidden" id="마스킹카드번호" name="마스킹카드번호" value="0000-00**-****-0000">
           </td><td class="t_left"><a href="#none" class="linkPoint2 letterType1" title="(유)딜리버리히어로코리아_카카오페이" onclick="popSlsl('merchant id2' , 'base64 value','2', 'pg id2', '', '', '', '')">(유)딜리버리히어로코리아_카카오페이</a>
           </td><td>9,700원
           </td><td>일시불   
           </td><td>      
           </td><td>0
           </td><td class="t_right">0 P
           </td><td> 전표매입
           </td><td>2021.04.01    
           </td>     
           <td>2</td>     
       </tr>
   
       <tr>
       <td>
           2021.03.02<br>23:59
           </td><td>
                <div class="popBalloon">
                    <span>CardIdent</span>
                    <p>CardCardCard</p>
                </div>
                <input type="hidden" id="마스킹카드번호" name="마스킹카드번호" value="0000-00**-****-0000">
           </td><td class="t_left"><a href="#none" class="linkPoint2 letterType1" title="Starlink Internet Serv" onclick="EngPopSlsl('base64 value','3')">Starlink Internet Serv</a>
           </td><td class="linethrough">$99.99
           </td><td>일시불   
           </td><td>      
           </td><td>0
           </td><td class="t_right">0 P
           </td><td> 취소전표매입
           </td><td>    
           </td>     
           <td>3</td>     
       </tr>
   
       <tr>
       <td>
           2021.02.28<br>23:59
           </td><td>
                <div class="popBalloon">
                    <span>CardIdent1</span>
                    <p>CardCardCard1</p>
                </div>
                <input type="hidden" id="마스킹카드번호" name="마스킹카드번호" value="0000-00**-****-0001">
           </td><td class="t_left"><a href="#none" class="linkPoint2 letterType1" title="GOOGLE* Domains" onclick="EngPopSlsl('base64 value','4')">GOOGLE* Domains</a>
           </td><td>$12.12
           </td><td>일시불   
           </td><td>      
           </td><td>0
           </td><td class="t_right">0 P
           </td><td> 전표매입
           </td><td>2021.04.01    
           </td>     
           <td>4</td>     
       </tr>
   
       <tr>
       <td>
           2021.02.26<br>23:59
           </td><td>
                <div class="popBalloon">
                    <span>CardIdent</span>
                    <p>CardCardCard</p>
                </div>
                <input type="hidden" id="마스킹카드번호" name="마스킹카드번호" value="0000-00**-****-0000">
           </td><td class="t_left"><a href="#none" class="linkPoint2 letterType1" title="Starlink Internet Serv" onclick="EngPopSlsl('base64 value','5')">Starlink Internet Serv</a>
           </td><td>$99.99
           </td><td>일시불   
           </td><td>      
           </td><td>0
           </td><td class="t_right">0 P
           </td><td> 전표매입
           </td><td>2021.04.01    
           </td>     
           <td>5</td>     
       </tr>
   
       <tr>
       <td>
           2021.02.24<br>23:59
           </td><td>
                <div class="popBalloon">
                    <span>CardIdent2</span>
                    <p>CardCardCard2</p>
                </div>
                <input type="hidden" id="마스킹카드번호" name="마스킹카드번호" value="0000-00**-****-0002">
           </td><td class="t_left"><a href="#none" class="linkPoint2 letterType1" title="쿠팡(쿠페이)" onclick="popSlsl('merchant id3' , 'base64 value','6', 'pg id3', '', '', '', '')">쿠팡(쿠페이)</a>
           </td><td>24,060원
           </td><td>일시불   
           </td><td>      
           </td><td>0
           </td><td class="t_right">0 P
           </td><td> 전표매입
           </td><td>2021.04.01    
           </td>     
           <td>6</td>     
       </tr>
   
       <tr>
       <td>
           2021.02.23<br>23:59
           </td><td>
                <div class="popBalloon">
                    <span>CardIdent</span>
                    <p>CardCardCard</p>
                </div>
                <input type="hidden" id="마스킹카드번호" name="마스킹카드번호" value="0000-00**-****-0000">
           </td><td class="t_left"><a href="#none" class="linkPoint2 letterType1" title="쿠팡(로켓와우월회비)" onclick="popSlsl('merchant id4' , 'base64 value','7', 'pg id3', '', '', '', '')">쿠팡(로켓와우월회비)</a>
           </td><td>2,900원
           </td><td>일시불   
           </td><td>      
           </td><td>0
           </td><td class="t_right">0 P
           </td><td> 전표매입
           </td><td>2021.04.01    
           </td>     
           <td>7</td>     
       </tr>
   
       <tr>
       <td>
           2021.02.21<br>23:59
           </td><td>
                <div class="popBalloon">
                    <span>CardIdent</span>
                    <p>CardCardCard</p>
                </div>
                <input type="hidden" id="마스킹카드번호" name="마스킹카드번호" value="0000-00**-****-0000">
           </td><td class="t_left"><a href="#none" class="linkPoint2 letterType1" title="(유)딜리버리히어로코리아_카카오페이" onclick="popSlsl('merchant id5' , 'base64 value','8', 'pg id2', '', '', '', '')">(유)딜리버리히어로코리아_카카오페이</a>
           </td><td>33,900원
           </td><td>일시불   
           </td><td>      
           </td><td>0
           </td><td class="t_right">0 P
           </td><td> 전표매입
           </td><td>2021.04.01    
           </td>     
           <td>8</td>     
       </tr>
   
       <tr>
       <td>
           2021.02.20<br>23:59
           </td><td>
                <div class="popBalloon">
                    <span>CardIdent</span>
                    <p>CardCardCard</p>
                </div>
                <input type="hidden" id="마스킹카드번호" name="마스킹카드번호" value="0000-00**-****-0000">
           </td><td class="t_left"><a href="#none" class="linkPoint2 letterType1" title="지에스25 어딘가점" onclick="popSlsl('merchant id6' , 'base64 value','9', '', '', '', '', '')">지에스25 어딘가점</a>
           </td><td>20,020원
           </td><td>일시불   
           </td><td>      
           </td><td>0
           </td><td class="t_right">0 P
           </td><td> 전표매입
           </td><td>2021.04.01    
           </td>     
           <td>9</td>     
       </tr>
   
       <tr>
       <td>
           2021.02.20<br>23:59
           </td><td>
                <div class="popBalloon">
                    <span>CardIdent</span>
                    <p>CardCardCard</p>
                </div>
                <input type="hidden" id="마스킹카드번호" name="마스킹카드번호" value="0000-00**-****-0000">
           </td><td class="t_left"><a href="#none" class="linkPoint2 letterType1" title="쿠팡이츠" onclick="popSlsl('merchant id7' , 'base64 value','10', 'pg id3', '', '', '', '')">쿠팡이츠</a>
           </td><td>24,500원
           </td><td>일시불   
           </td><td>      
           </td><td>0
           </td><td class="t_right">0 P
           </td><td> 전표매입
           </td><td>2021.04.01    
           </td>     
           <td>10</td>     
       </tr>

        </tbody>
</table>"###;

        let result = parse(table);

        assert_eq!(result.0.iter().any(|x| x.card_no == "0000-00**-****-0000"), true);
        assert_eq!(result.1.iter().any(|x| x.amount == "3906"), true);
        assert_eq!(result.1.iter().any(|x| x.merchant == "(유)딜리버리히어로코리아_카카오페이"), true);
    }

    #[test]
    fn parse_amount_test() {
        assert_eq!(("1234".into(), "원".into()), parse_amount_currency("1,234원"));
        assert_eq!(("99.99".into(), "$".into()), parse_amount_currency("$99.99"));
        assert_eq!(("123.45".into(), "€".into()), parse_amount_currency("€123.45"));
    }
}
