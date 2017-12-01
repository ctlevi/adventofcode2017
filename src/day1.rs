static INPUT: &'static str = "6592822488931338589815525425236818285229555616392928433262436847386544514648645288129834834862363847542262953164877694234514375164927616649264122487182321437459646851966649732474925353281699895326824852555747127547527163197544539468632369858413232684269835288817735678173986264554586412678364433327621627496939956645283712453265255261565511586373551439198276373843771249563722914847255524452675842558622845416218195374459386785618255129831539984559644185369543662821311686162137672168266152494656448824719791398797359326412235723234585539515385352426579831251943911197862994974133738196775618715739412713224837531544346114877971977411275354168752719858889347588136787894798476123335894514342411742111135337286449968879251481449757294167363867119927811513529711239534914119292833111624483472466781475951494348516125474142532923858941279569675445694654355314925386833175795464912974865287564866767924677333599828829875283753669783176288899797691713766199641716546284841387455733132519649365113182432238477673375234793394595435816924453585513973119548841577126141962776649294322189695375451743747581241922657947182232454611837512564776273929815169367899818698892234618847815155578736875295629917247977658723868641411493551796998791839776335793682643551875947346347344695869874564432566956882395424267187552799458352121248147371938943799995158617871393289534789214852747976587432857675156884837634687257363975437535621197887877326295229195663235129213398178282549432599455965759999159247295857366485345759516622427833518837458236123723353817444545271644684925297477149298484753858863551357266259935298184325926848958828192317538375317946457985874965434486829387647425222952585293626473351211161684297351932771462665621764392833122236577353669215833721772482863775629244619639234636853267934895783891823877845198326665728659328729472456175285229681244974389248235457688922179237895954959228638193933854787917647154837695422429184757725387589969781672596568421191236374563718951738499591454571728641951699981615249635314789251239677393251756396";

pub fn solution() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> u32 {
    let mut result = 0;

    // Create another string from the input, shifted with the first character removed and put at
    // the end. This will let us zip the two together and get the next digit in the list easily
    // example: input=         12345
    //          shifted_input= 23451
    let first_character = input.chars().nth(0).unwrap();
    let mut shifted_input = input[1..].to_string();
    shifted_input.push(first_character);

    let digits = input.chars().map(|c| c.to_digit(10).unwrap());
    let next_digits = shifted_input.chars().map(|c| c.to_digit(10).unwrap());

    for (current, next) in digits.zip(next_digits) {
        if current == next {
            result += current;
        }
    }
    return result;
}

fn part2(input: &str) -> u32 {
    let mut result = 0;

    // Create another string from the input, shifted with the first half of the characters
    // after the second half of the characters. This will let us zip the two together and
    // get the next digit in the list easily
    // example: input=         1234
    //          shifted_input= 3412
    let first_half = input[0..input.len() / 2].to_string();
    let second_half = input[input.len() / 2..].to_string();
    let shifted_input = second_half + &first_half;

    let digits = input.chars().map(|c| c.to_digit(10).unwrap());
    let next_digits = shifted_input.chars().map(|c| c.to_digit(10).unwrap());

    for (current, next) in digits.zip(next_digits) {
        if current == next {
            result += current;
        }
    }
    return result;
}


#[cfg(test)]
mod tests_part1 {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(3, part1("1122"));
    }

    #[test]
    fn example2() {
        assert_eq!(4, part1("1111"))
    }

    #[test]
    fn example3() {
        assert_eq!(0, part1("1234"))
    }

    #[test]
    fn example4() {
        assert_eq!(9, part1("91212129"))
    }
}

#[cfg(test)]
mod tests_part2 {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(6, part2("1212"));
    }

    #[test]
    fn example2() {
        // 1221
        // 2112
        assert_eq!(0, part2("1221"))
    }

    #[test]
    fn example3() {
        assert_eq!(4, part2("123425"))
    }

    #[test]
    fn example4() {
        assert_eq!(12, part2("123123"))
    }

    #[test]
    fn example5() {
        assert_eq!(4, part2("12131415"))
    }
}
