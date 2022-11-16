use nom::character::complete::{alpha0, char};
use nom::combinator::rest;
use nom::sequence::preceded;
use nom::IResult;

#[derive(Debug, PartialEq, Eq)]
pub enum Message {
    Empty,
    ChallStr(String),
}

fn pipe(i: &str) -> IResult<&str, char> {
    char('|')(i)
}

fn msg_type(i: &str) -> IResult<&str, &str> {
    preceded(pipe, alpha0)(i)
}

pub fn parse(input: &str) -> IResult<&str, Message> {
    let (input, msg_type) = msg_type(input)?;
    match msg_type {
        "challstr" => {
            let (input, challstr) = preceded(pipe, rest)(input)?;
            Ok((input, Message::ChallStr(challstr.to_string())))
        }
        _ => Ok((input, Message::Empty)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const UPDATE_USER: &str = r#"|updateuser| laqerhill|1|169|{"blockChallenges":false,"blockPMs":false,"ignoreTickets":false,"hideBattlesFromTrainerCard":false,"blockInvites":false,"doNotDisturb":false,"blockFriendRequests":false,"allowFriendNotifications":false,"displayBattlesToFriends":false,"hideLogins":false,"hiddenNextBattle":false,"inviteOnlyNextBattle":false,"language":null}"#;

    const CHALLSTR: &str = r#"|challstr|4|0bfe5841e78f6503ad529828a468cb10557c7b836f770914bedfdf74763337158568c977f01d6fa6f33acf6bf2e67293dd80788092341a46569c3987dcb51cb98473330a2061bc4dff633117f5a51f6ef4e2f8e06daf627e6a1e0cdf07933478b2795754268d5ad1ba2fc37ef1983422a615898a4abe0ffd6afabcaa430a217c"#;

    #[test]
    fn test_msg_type() {
        assert_eq!(
            msg_type(UPDATE_USER),
            Ok((
                r#"| laqerhill|1|169|{"blockChallenges":false,"blockPMs":false,"ignoreTickets":false,"hideBattlesFromTrainerCard":false,"blockInvites":false,"doNotDisturb":false,"blockFriendRequests":false,"allowFriendNotifications":false,"displayBattlesToFriends":false,"hideLogins":false,"hiddenNextBattle":false,"inviteOnlyNextBattle":false,"language":null}"#,
                "updateuser"
            ))
        );

        assert_eq!(
            msg_type(CHALLSTR),
            Ok((
                r#"|4|0bfe5841e78f6503ad529828a468cb10557c7b836f770914bedfdf74763337158568c977f01d6fa6f33acf6bf2e67293dd80788092341a46569c3987dcb51cb98473330a2061bc4dff633117f5a51f6ef4e2f8e06daf627e6a1e0cdf07933478b2795754268d5ad1ba2fc37ef1983422a615898a4abe0ffd6afabcaa430a217c"#,
                "challstr"
            ))
        );
    }

    #[test]
    fn test_parse_challstr() {
        assert_eq!(
            parse(CHALLSTR),
            Ok((
                "",
                Message::ChallStr(
                    r#"4|0bfe5841e78f6503ad529828a468cb10557c7b836f770914bedfdf74763337158568c977f01d6fa6f33acf6bf2e67293dd80788092341a46569c3987dcb51cb98473330a2061bc4dff633117f5a51f6ef4e2f8e06daf627e6a1e0cdf07933478b2795754268d5ad1ba2fc37ef1983422a615898a4abe0ffd6afabcaa430a217c"#.to_string()
                )
            ))
        )
    }
}
