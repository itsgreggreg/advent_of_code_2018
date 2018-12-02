fn main() {
    let input = str_input();
    let input2 = str_input();
    let mut skip = 1;
    let mut same_chars = None;

    for line1 in input.lines() {
        if same_chars != None {
            break;
        }
        let mut current_line2 = 0;
        for line2 in input2.lines() {
            if same_chars != None {
                break;
            }

            same_chars = only_one_difference(&line1, &line2);

            current_line2 = current_line2 + 1;
        }
        skip = skip + 1;
    }

    if same_chars != None {
        println!(
            "The matching lines chars are {}",
            same_chars.unwrap_or(String::from(""))
        )
    }
}

fn only_one_difference(left: &str, right: &str) -> Option<String> {
    let mut total = 0;
    let mut left_chars = left.chars();
    let mut right_chars = right.chars();
    let mut same_chars = String::from("");

    loop {
        match (left_chars.next(), right_chars.next()) {
            (Some(l), Some(r)) => {
                if l != r {
                    total = total + 1;
                } else {
                    same_chars.push(l);
                }
            }

            _ => break,
        }
    }

    if total == 1 {
        Some(same_chars)
    } else {
        None
    }
}

fn str_input() -> String {
    String::from(
        "\
qywzphxoiseldjrbaoagvkmanu
qywzphxeisulpjrbtfcgvkmanu
qywzxhooiseldjrbtfcgvcmanu
qywzphjojseldjubtfcgvkmanu
qtwzphxoieeldjrbtfcgvrmanu
tywzphzoiseldjritfcgvkmanu
qyuzphxoiseldjrbtfcgykbanu
qswzmhxoiseldjrbtfcgvkaanu
qyczqhxoiseldjrbtfcgvkbanu
qybzpqxooseldjrbtfcgvkmanu
qywzpoxoxseldjrbtfcgvpmanu
qywzphxbipeldjrbtfcgvkmaru
qywzpmxoiseldjrutqcgvkmanu
qywzphxoisesdjrrtfchvkmanu
qywzphxoiseldhrbtfcgukyanu
jywzphyoiseldjrbtfclvkmanu
qywzvhxoiselrjrbtfcgvkmanl
qywzplxojseldjrbtfcgvkmamu
qywzphxoiseldjrbtacgvkmvnd
qywpphxoiseldjrbtfcgvkvenu
qqazphxoiselqjrbtfcgvkmanu
qyozvhxoiseldjrbtfcgvkmadu
xywzphxliseldjhbtfcgvkmanu
qdwzphioiseldqrbtfcgvkmanu
qswyphxoiseldjrbtfcgvkmanx
qowzshxoiseldjrbtfigvkmanu
qywzphxoiseldjjbtfcgvkmawq
qyhzphxoiseldjrbtfvgvkmynu
qxwzphxoiselmjrbtfcgvqmanu
qywzphxoiceldjrbtfcgikmgnu
qywephxoiseldjrbthcgvkmane
yywzphxoisejdjrotfcgvkmanu
qywzxhxoisyldyrbtfcgvkmanu
qywzpjxoiseqdjrbtfcgvwmanu
qywzphxoqweldprbtfcgvkmanu
qywzphnoiseldjrbtfcamkmanu
qylgphxoiseldjrbtfcgvkmauu
qysdphxomseldjrbtfcgvkmanu
lywzpoxoikeldjrbtfcgvkmanu
qywzphxgiseldjrbtfeqvkmanu
qywzdhxozseldjcbtfcgvkmanu
qywzphxeiskedjrbtfcgvkmanu
qmwzphxoispldjrbtfcgvkmany
qywzphxoiselzcrbtfcgvkmanz
qywzphxoisxodjrbtscgvkmanu
qywzphxoiseldjrrtfcgmkmaeu
aywzphxoiseldjrbtfmjvkmanu
qywzthxoiiewdjrbtfcgvkmanu
qywzpsxoisrzdjrbtfcgvkmanu
qywzphxoiseldjkbtqcgvkmank
tywzphxoisewdjkbtfcgvkmanu
qywkchxoiseldjrbtfcghkmanu
qywzphxoiseldjoftfrgvkmanu
qywzpixoisehdjrbtfcgvkmacu
qywzchooiseldarbtfcgvkmanu
qywyphtoaseldjrbtfcgvkmanu
qywyphxotsbldjrbtfcgvkmanu
qywzphxoiseldjrbhsggvkmanu
dywzphxoiseldjrktfegvkmanu
qywzphxoiseldjrmtfcgvkcenu
qywcphxjiseldjybtfcgvkmanu
qywzphxoiseldhrbsfogvkmanu
qmwzpheoiseldjrrtfcgvkmanu
qywzrhxoiselpjrbtfcgvemanu
qyezphpoiseldjrbtfcgvdmanu
qywzphxvisewdjrbtfcgvkmdnu
qywzphkoiseldjrbtfcgvkmdnh
qywzehxoiseldfrbtpcgvkmanu
qywzphxoiseldjrrtfcgvrdanu
qpwzphxoizeldjqbtfcgvkmanu
qywzphxojseldjrbtmcgvkmvnu
vywzphxoiseldjrbtfcgvkmaop
sywzphxoiselbjrttfcgvkmanu
qywwphkoiseldjrbtfcivkmanu
qlwkpcxoiseldjrbtfcgvkmanu
qywzphxoiyesdjrbtfcgvkmvnu
qywzphxoiseldjrbofcgrkmrnu
qywzphxyiseldjrstfcgvkmjnu
qywzphaoiseldtrbnfcgvkmanu
qywzphxhisuldurbtfcgvkmanu
qywzphxdiseldjrbtvugvkmanu
qywzpzxbiseldjrbtfcgukmanu
qyrzphxoixeldjrbtfcgvumanu
qywzphxoiberdjrptfcgvkmanu
qywzphxfiskldjrbtfcgvkmdnu
qyxzphxoiseldjrdhfcgvkmanu
qywzphrqiseldjrbtfcgvbmanu
qyezphxviseldjrbtfcgvkmani
qywgphxoiseldgkbtfcgvkmanu
qywzphxoileldjrbtgcgvkdanu
qywzphxoiseldnrbtfcwvkmpnu
qywzphxoiseidjrbmfcqvkmanu
qywzpkxoiselijrbtfcgvlmanu
mywzphxoiveldjrbtfcgvkmunu
qywzphxooseddjrbtfcgpkmanu
qywzphxokseldjritfcyvkmanu
qywzxhxoiseldjrbtfqgvcmanu
qywzphxoisfldjrbpfcgvkmaju
qywuphxgiseldjrbffcgvkmanu
qywzphociseldjrbtfcgvkuanu
qywzphxoiseldvrbtftgckmanu
qywzpbxoisrldjrbtfngvkmanu
qywzphxoiseldjrntfygvdmanu
qywzphxviseldkrbtfcgvkianu
qywpphxgiseldjrbtfctvkmanu
qywzphxoicewdjrbtfcgvsmanu
qywzpcxoiseldjmbtfcgvcmanu
qrwzphxoiseldjrbtfcgjumanu
qywzphxoiselojrbtfcgxkmaau
qywzphxojsbldjrbtfcgykmanu
oywzphxoiseldjrbtfqgvkmvnu
qywfphxpiseldjrbtfckvkmanu
qyyzwhxwiseldjrbtfcgvkmanu
qywzphxgiseldjrbtfchvkmabu
qywzphxfiseldjrbtfcgukoanu
qywzpdxoisyldjrbtfcgvkxanu
dyuzphxoiseldjrbtfcgvkmamu
qywzphxoiseldjrbifcgvkmnnp
qywzpyxoiseldjrbtfcgvklano
dywzphxoiieldjrbtfcgvwmanu
qywzphxoihemdjrbtfcgvdmanu
gywzphxoxseldvrbtfcgvkmanu
qywzqhxoissldjwbtfcgvkmanu
eywzphxoiieldjrbtfcgekmanu
qyizprxciseldjrbtfcgvkmanu
qywjphxoiseldjrbtfcgckmano
qywznhxoiseldjrbrfcgvkmagu
qywzphxoisrldjdbvfcgvkmanu
qyyxphxoiseldjrbtwcgvkmanu
qywzphxoiseldjdbtfcpvkmjnu
qywzvhxqiseldjrbofcgvkmanu
vywzphxoiseldjrbtfcgckwanu
qywzphgbiseldjrbtfcgvkmazu
qcwzphxoiseldjrbqfcgvkmdnu
qywzphxoismldjrbtfcgkkmznu
qywhphxoiseldjrbtccgvkmane
qywzphzoiseldjrbtfcgvqmauu
hywzphxoiseldjrbtfcuvkmanc
qywzphxozsejdgrbtfcgvkmanu
qyszphxoiseldjrntfygvdmanu
qywzphxoisgldjrbtfcgvklaru
qywzhhxoiseldjrbtscgvkmqnu
qywjphxpiqeldjrbtfcgvkmanu
qywzphxoiseldxrptfclvkmanu
qywlphxoisehdjrbtfcgvkmanc
qydzpfxoiseldjrwtfcgvkmanu
qywzphxoiseldjrbtxcgqkfanu
qywophxoiselfjrbtfcgvkmani
qywzyhxoiszldtrbtfcgvkmanu
qywzphxoxseldfrntfcgvkmanu
qywzphloiseldjqbtfcgvkmtnu
qywzpuxoiseldorbtfcgvkeanu
qywzphxoiueldjrwdfcgvkmanu
qgwzphxoiseldjmbtncgvkmanu
qywzphtggseldjrbtfcgvkmanu
qywzphxoisrldjrbtfmgvhmanu
qfszphxoiseldjqbtfcgvkmanu
qywzphxpisjldjrbxfcgvkmanu
qywznhxoisepdjrbtfqgvkmanu
qywzphioiseldjabtfcgxkmanu
qyizphxaiseldjrbtfcgvkmaxu
xywzphxoiqelvjrbtfcgvkmanu
quwzphxoiseldjretfcgvkmaau
bywzphxoiseldjrbtucgckmanu
jywzphxoiseldjrbofcgvkmani
qywzphxoiseltjkbtfcgvkmabu
eywzphxoiselgjrbtfkgvkmanu
qywzphxoisengjrttfcgvkmanu
qywzphzoiseldjrbtfcgvkmknk
quwdphxoiseldjrbtfcxvkmanu
qcwzzhxoiseldjrgtfcgvkmanu
qywgphxdiseldjrbtfcgjkmanu
qywzpdxoivefdjrbtfcgvkmanu
qywzphxoiseldjrbtfdgvjmpnu
qfwzphxoiseydkrbtfcgvkmanu
qywzphxdiqelqjrbtfcgvkmanu
qywzvhxoiseldjrbtfognkmanu
qywzphgoiseldjrbcfcgvtmanu
qywophxoiseldjrbtpcgvkmank
qywzphxoiszldjretfcgvkmabu
qywzphxoiseldjhbtfcgvxmawu
qcgzphxoiselejrbtfcgvkmanu
qywzphxoisepdjrbtfcfvkcanu
qivzphxniseldjrbtfcgvkmanu
qywzhhxoiseldjrftxcgvkmanu
qyazphxciseldjrbtfcgskmanu
qywzphxoisoldgrbtfczvkmanu
qywzmhxoiseldurbwfcgvkmanu
qywzphxoistldjrbwfcgvkranu
qywzphxoistedjrbtfcgokmanu
qywzqhxodsecdjrbtfcgvkmanu
qywzphxtisxldjrbtfcgvkhanu
qywzphxoeseldjrtrfcgvkmanu
qdwzphxoioeldjrbtfigvkmanu
qjwzphxoisbldjrbtfcgvkmanz
qywzphxoiseldbrbtfdgvlmanu
qywzphxoiselddrbhvcgvkmanu
zywzppxoiseldjrdtfcgvkmanu
qywzppxqiselkjrbtfcgvkmanu
qywzphxoihelbjrbtfcgvkmabu
qywzphxoiseldjreyfcgvknanu
qywzphxxrseldjrbtfcgvkmagu
qywhpfxoiseldjrbtfdgvkmanu
qywzphxoisxldjrdtfagvkmanu
oywzphxxiseldjrbtfcgvkmaeu
qywzphxoiselqirbtfvgvkmanu
qywzphxoqseldhrbtfcgvkhanu
qywzphxokseldjrbtfxgvkmaju
qywzphtoiseldurbtfcfvkmanu
qywzphxoiheudjrrtfcgvkmanu
qrwzphxzigeldjrbtfcgvkmanu
qywzphxoiseldorbtfcgvxmvnu
qywzphxoisaldjpbqfcgvkmanu
qywuphxoiselljrbtfchvkmanu
qywzphxoisaldjrbefcgvkmsnu
qywzphxoiteldjrbnfcgvkmanp
qywhphxoiselqjrbtfcgvkmagu
qywzjhxoisejdjrbtfcgvkmanr
qywephaoiseldjrbtfcavkmanu
qywcphxoireldjqbtfcgvkmanu
qywzphxoiseldirbuicgvkmanu
qywzphxoisecvnrbtfcgvkmanu
qcwzphxoiseldjrbtfcgvrmaiu
qywnphxoiseldjrntftgvkmanu
qywzphxhisyldjrbtfcgvkmafu
qyhzphxoiseldjrytfcgvkmanq
vjwzbhxoiseldjrbtfcgvkmanu
qyvzpsxoisuldjrbtfcgvkmanu
qywzphxaiseldcrbefcgvkmanu
qywzphxoiseldjgbtfsgvkfanu
oiwzphxoiseldjrbtfcgvkmcnu
qyezphxoiseldjrbtfcqvkmjnu
tywzphxriseldzrbtfcgvkmanu
yywzphxoiseldjbbtfugvkmanu
qywzpfxviseldjrbttcgvkmanu
qywzphxoiceldcrbtfugvkmanu
qymzphxoiseldjratfcsvkmanu
qywzphxoiselxjrbdfcgvkpanu
qywzphxoiselujrbtfkgvimanu
qywzshyoiseldjrbtfcgpkmanu
qywzphxoiselfjrbtfsgvkmant
qywpphxoiseldjxbtfcyvkmanu
qywzfhxoiselqjrptfcgvkmanu
qewzphxoiseldprbtfcgvkmand
qywfphxoiseldlrbtfcgvkmgnu
qywzphxoiseldjhbtqcovkmanu
fywzphxoiseldlrbtfcgvkjanu
sywzphxoiseldjrbhfccvkmanu
qywzphxoiseldjfbtfcrvkmpnu
sywzphxoisrldjrbtfczvkmanu
",
    )
}
