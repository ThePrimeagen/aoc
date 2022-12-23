use std::{collections::HashSet, thread::JoinHandle, sync::mpsc::Receiver};

const PROD: &'static str = "qvllndllhzhfzhhdzhddhjdjggvnvhvccmffwllqgqmmfjfqfhhtrrzczjczzlplddfpptqqfbqffmnmjnnqppfjfccgnnmqqsvvdbbgppjvpjvpjjctjjttwtrrdldlcddrvddqndqnqwqwzwfwwzczggcppgzpzhpzhppprfffbhhwmhhtftstrsrvsrvsrvvshvssnwwpllhfhnnfflcltlblzlqlvqlvlcldcccpptggtdgdjdbbrggmbmnncscbssqrrjddvcvgvfflpppgpvphvphhpcpzpzvvctvctvthtwtfwwbrrhhlplmlwwlqlnlhhtmhmmqlqplllrvrgvrvrffzfgfjfjtjmjvmjmwmvvjffmpfphfhvfvmfvmmhpphhltthgttgccqggpzpfpqpcpvcpvcvvqtqvqbbrlrtllmrmllhmhvmhhvzhvzvrrrzjzbbtvvbgvbbfnnqndqnnpnbnbnlnggwqggmgmqmgmbbmccgqcqbccpvcvnnhvvrvlrrcwrcwrcwrwbrwwzbwbdbfddpttntzzjszsnznbndnzngzgccjrcjchcffmlmqqlrqqzsqzzsbsnsttzpztpzpggzrrttbqqplpqlqjjqcqvccdzdccthccvfcvvqvhqhfhhzwzpzwppgpttntssflfjjrwrqrjrppptlltptpvttpfpwpswpppzzsrzssqllbnlljpllrjllsrlrhrdrmdrmrrpsrprnrffgrffdqdhdqhhrhggwqqlddsbsqbqtqdtdhdvhhbdhdzhdhhtrrppzddgfgzgpzpvpfpnpptggltggbnbppqffzfrzzzsbsrrdgrddwsdsqddhpdpbpvpfvppfsfgfngffzmzbzlblclsccvqvqmmjtjqtjjlcjllsddjqddhldlvlrrbgbrgbrrdzzpfpggqnqbqrrqbbgjgppqgpgwgqqndncndnpdnnbvbnvnwnjjgppzlplqqdgqghqgqzggjssqmmwwcfcpptrpprggrppgbplmzwmdtnpqwzcrthqbppwbgcvgqrpfpnbscnhvrllpvpqwnsslcjrqtvdccprvqfrpswtpvzdzlgtmmvppdmhgdbbsmrbqpqspdhpqgfjznqzphrnggcbzhdqrgvzcfzrhtrlssgmjjghqsjtghhnwjffqrrfslfnsvvdvfjqbfpffrrstdhggvbfwtfpfgswqlfdrnjpjmwzptlbmwgghgwqrphcrvfmhrplllgbnjlprllmjwccphsflntgpnbmdbfqcdsbgvrnfznfrlcfvswqfrqvdnbjsflnsmlcrdstzppmcvbgdtcvgztbdzqbwhmwcfvbwjjcdgbnwjwzrrdqhpgscwtnztjsfstzfwftcldjgvdvwbzrlbdslwttbqpnlwbjcjwqgtrgcglsgtdqbqbnqznptzzbwffwlwzvvtdpcjbvhnswzptclpbndcdvsfmcrmwwgzdfsszqjjdztmtsqgfqzjpctfdpwnzbpnzzwngqnghntblndfrnjzdrmgbqmzbdqfzctrgshwqgfgqssqjltrqlzjswjhmpgwwjdwcjpnsvgrvbfpmlmmwzmbdjwsrjthppfrccjgnmwlvqlprgslbwtbbzlqbznczmsmhsfdcqnwblprcpbzzwfllbnldvpjcwsdhglrzjsptmsjdjqzsmgvhjfjrrtvvbjlmzjsntnrggwbpjlrjggfgqzvswtggthzfmfjnmrzrttbzqpwpsnmdtnbfblpfgslgcmjlbdpshnnrbhvwsbrnvdmjqhvhdjhbfzjmqrmqmdthhzvnrmqcnbtwcdjdqfvdgvmfbhrfqnmdncrddggtcppjlznbsnntppjtnsqsrjwvfrzpnzqcrzhhdflfmmtmwcvtpzbqhdwsczffcqhtdbdjblmgnrmhlqcsvcpgghhvwqhdtzpzlpfllchzltqgcwgfqnbzhgzmdwqdlwnvhqmpqjqnjbhjctslghdqvctdmjfwdfpdjnhdndzwsfjzlmsbmfmzvnvpqgqhtngvgqmlrrzsfmwlcwsscvghjvrzjjqbnplnjzqswpblwzwczhwbhhnjmctnmwlbqqfmnlwdcrptlmfjpjrnpcvmhffjhwhmntdzpdjzwzhrrsdvmjlwdtcpvjfmfzfsrgjghhlvmjjjczgmhvrfpgqbnhldwbrjgzmnszzbssfzcggrwmdfvddwsdmnwtwfwlfnwlvzlctfblbtrjvcwjjdljplcrjhwqslppwwtvfqwsjlfmdznmcdzdmgvmmsrfcclcvhtrhlsjzrbjwrjlfnvqhqvmpzmdttnbhfcvnqlrqbcsvtvwfccjstjpmhqgwlnrzjjmfdszflmglrdbpqhqhqsdfzrcljbdvvnlcqfllmnqcjfzjppdsjwshfschzqbnwfqnpwhqnmwsjbtcgvrljsrtzvcvghcjjlqsngglcggqpntrrhbjpbfhmvpltmnfmfdtwnczwfbvjcqnhvppjftwvwsrlhvvcjtsfptpqgrmrqwwddnqmnmfgrlnphbpqhhhvglqgtwvnwvnbssftmwttmfrffwtzhrpqspclvgchwqwcsgwqwwvpgcwngrcfmhbhflwfbfchlphdzdcrflfmfclsngtlwrqcrsgrdzcpdsvvcdbhgtljmbntbbcqgjqfsbfwzlfsnljpjdcnmjlqrwpmlvwgdlrrdgfhdqhzgltmclzgzzhmrbggsmgtpqdrgmjtlzwstrwbpvhppvsmdqvvwwglzjgdswjszqmrdbmshbhhcstpcsjdbvgjnvcmvhbtclrlmlgnvppgvncsrfchdbqjrclwwlnchmcgvshfsbsvvcvjrsgjlnsfqtqmgntffwnqjtldcqbcqhsgztllstswwqnfrswpchqhnfzzzszqjztzfrgrbjdbjlpvqfqrlrmmpbfbbcclrgmnlzwqrjhqrstswjpgsrtnlwsbqthzpvdzllzqmdmbvvtcztftvlwphhjzbfnrvccfmhmvmzlbrzlnppfzcsffjvjmbgpvlwgwszpztjpsrbnftqtdrbnljtbrjzzbwlsvtwtlwptdtnmtncvcblcmdngjzmctlqtzchncccnwjzrrmmmnllbhrnhwtqjsnvcslrqjfbfndqvdlrjshdzmlprtzbtnhthdqhplwzdbnjmgzlzrbzrvrqnflwfmsmbssqnbcddnvdpltpmplpdzvtjrslcdcnrdplwtjtvctwfzhlvwwqqtbqcjjwhhnpmvgzhqmqfgthwbphrmrtdghchsmwghdqjgjgmpddbrtngtvhqgjfrplrdgpbnhqvswrmqhcmsqvsqmqsgwjndwjrbrhvrctmmrmfwpsgfgdlrzpslpflgvwrgcthgcrnhgrzsmqdgdssjgspfhmqfmjfpmwqhnfjdvqzhpndvnbmqglbrjmdrwgmgctrgzpsdvfbmcstcslblmvnprphntgslmlrqwthrndrhtbccgzzfsglhgqztcsnqjwfzbzlvrpbvswbhrwdsrhrrpnrmsbvbvjccbdsdcfrrzpgwjtnnnvjwlcppwzdqsbdzpfjplrlfgvjpsmbzwpwlghnvqgddfjvrsztrpzlfgmqqzrfcgglghndbhgbmldglclhldljjdslvhzshshtqwhqnbzhvqrcmwdmcmhjcrmdmhrwnwcbhvbbrwrbtfdnztwnbpdfjfhgrmcpngftsvbsmsptnwcvvllnmbnsntbzmwnhfdptbtzswtjzdqwjdhprnjwvhzpscjvlsgrhdrmmrmhzhwwtslzdjqmzfncnmgplhnmwrvqhslvchtjcmpzpjpnpfbjptvvwcsmhgdjtsqrjlfpnfdncpqqmpgpvtlvwljlsqbnhtsqgfwlsmdjpgtvgjvjcrnnzmbllqzlrfdnlffgmtphhhgbcjgdlpzqpwmjwtcmdrsmtnmddftwczbsddtppsptbwfvpnfnsqmsgcfqfmnzffzqgcdvwzrgdwhmnzmrlhcdpdsltnsmjzdqwmmpwvjqbbwsrfgzh";

fn join(rx: Receiver<Option<usize>>, max: usize) -> Option<usize> {
    let mut found = 0;
    while let Ok(x) = rx.recv() {
        if let Some(x) = x {
            return Some(x);
        }
        found += 1;

        if found == max {
            return None;
        }
    }

    return None;
}

fn to_string(data: &[u8], start: usize, stop: usize) -> String {
    return data[start..=stop].iter().filter_map(|x| char::from_u32(*x as u32)).collect::<String>();
}

fn simple(i: &str, num: usize) -> usize {
    let i = i.as_bytes();
    return i.windows(num)
        .position(|w| {
            return w.iter().collect::<HashSet<_>>().len() == num;
        })
        .map(|x| x + num)
        .unwrap();
}

fn faster(i: &str, num: usize) -> usize {
    let i = i.as_bytes();
    return i.windows(num)
        .position(|w| {
            let mut hash_set = HashSet::new();
            for x in w {
                if !hash_set.insert(x) {
                    return false;
                }
            }
            return true;
        })
        .map(|x| x + num)
        .unwrap();
}

fn fastester(data: &[u8], num: usize, offset: usize, length: usize) -> Option<usize> {
    let mut idx = offset + num - 1;
    let data_len = offset + length;

    while idx < data_len {
        let mut left_len: usize = 0;
        let mut left_idx: usize = idx;
        let mut left: usize = 0;
        //let print = true;

        loop {

            // we have found the idx
            if left_len == num {
                return Some(idx + 1);
            }

            let res = left ^ (1 << (data[left_idx] - b'a'));
            if res < left {
                break;
            }

            left_idx = left_idx - 1;
            left_len += 1;
            left = res;
        }

        let mut right_len: usize = 0;
        let mut right: usize = 0;
        let mut right_idx: usize = idx + 1;

        while right_idx < data_len {

            // we have found the idx
            if right_len == num {
                return Some(idx + right_len + 1);
            }

            let res = right ^ (1 << (data[right_idx] - b'a'));
            if res < right {
                break;
            }

            right_idx += 1;
            right_len += 1;
            right = res;
        }

        let len = right_len + left_len;

        if len == num && (right ^ left).count_ones() == num as u32 {
            return Some(right_idx);
        } else if len > num {
            if let Some(x) = fastest(&data[left_idx..=right_idx], num) {
                return Some(x + left_idx);
            }
        }

        idx += num;
    }

    return None;
}

fn fastesterest(data: &[u8], num: usize, offset: usize, length: usize, start_offset: usize, jump_amount: usize) -> Option<usize> {
    let mut idx = offset + num * start_offset - 1;
    let data_len = offset + length;

    while idx < data_len {
        let mut left_len: usize = 0;
        let mut left_idx: usize = idx;
        let mut left: usize = 0;
        //let print = true;

        loop {

            // we have found the idx
            if left_len == num {
                return Some(idx + 1);
            }

            let res = left ^ (1 << (data[left_idx] - b'a'));
            if res < left {
                break;
            }

            left_idx = left_idx - 1;
            left_len += 1;
            left = res;
        }

        let mut right_len: usize = 0;
        let mut right: usize = 0;
        let mut right_idx: usize = idx + 1;

        while right_idx < data_len {

            // we have found the idx
            if right_len == num {
                return Some(idx + right_len + 1);
            }

            let res = right ^ (1 << (data[right_idx] - b'a'));
            if res < right {
                break;
            }

            right_idx += 1;
            right_len += 1;
            right = res;
        }

        let len = right_len + left_len;

        if len == num && (right ^ left).count_ones() == num as u32 {
            return Some(right_idx);
        } else if len > num {
            if let Some(x) = fastest(&data[left_idx..=right_idx], num) {
                return Some(x + left_idx);
            }
        }

        idx += num * jump_amount;
    }

    return None;
}

fn fastesterest_sync(data: &[u8], num: usize, jump_amount: usize) -> Option<usize> {
    for i in 1..=jump_amount {
        if let Some(x) = fastesterest(data, num, 0, data.len(), i, jump_amount) {
            return Some(x);
        }
    }

    return None;
}

fn fastest_async(data: &'static [u8], num: usize, count: usize) -> Option<usize> {
    let (tx, rx) = std::sync::mpsc::channel();
    let jump_amount = data.len() / count;
    for i in 0..count {
        let inner_tx = tx.clone();
        std::thread::spawn(move || {
            let start = if i == 0 { 0 } else { i * jump_amount - num + 1};
            let mut stop = start + jump_amount;
            if i == count - 1 {
                stop = data.len();
            }

            _ = inner_tx.send(fastest(&data[start..stop], num));
        });
    }

    return join(rx, count);
}

fn fastesterest_async(data: &'static [u8], num: usize, jump_amount: usize) -> Option<usize> {
    let (tx, rx) = std::sync::mpsc::channel();
    for i in 1..=jump_amount {
        let inner_tx = tx.clone();
        std::thread::spawn(move || {
            _ = inner_tx.send(fastesterest(data, num, 0, data.len(), i, jump_amount));
        });
    }

    return join(rx, jump_amount);
}

fn fastester_async(str: &'static str, num: usize) -> Option<usize> {
    let data = str.as_bytes();
    let cpus = 12;
    let regions = data.len() / cpus;
    let (tx, rx) = std::sync::mpsc::channel();
    for i in 0..cpus {
        let start = if i == 0 { 0 } else { i * regions - num + 1 };
        let inner_tx = tx.clone();
        let mut len = regions;

        if i == regions - 1 {
            len = data.len();
        }

        std::thread::spawn(move || {
            _ = inner_tx.send(fastester(data, num, start, len));
        });
    }

    return join(rx, cpus);
}

fn fastest(data: &[u8], num: usize) -> Option<usize> {
    let mut filter = 0u32;
    data.iter()
        .take(num - 1)
        .for_each(|c| filter ^= 1 << (c - b'a'));

    return data.windows(num)
        .position(|w| {
            let first = w[0];
            let last = w[w.len() - 1];
            filter ^= 1 << (last - b'a');
            let res = filter.count_ones() == num as u32;
            filter ^= 1 << (first - b'a');
            res
        })
        .map(|x| x + num);
}

fn main() {

    let count = 14;
    let string = std::fs::read_to_string("long").unwrap();
    let string: &'static String = Box::leak(Box::new(string));
    //let string = PROD;
    let bytes = string.as_bytes();

    let now = std::time::Instant::now();
    let result = simple(string, count);
    println!("simple({}): {}", result, now.elapsed().as_millis());

    let now = std::time::Instant::now();
    let result = faster(string, count);
    println!("faster({}): {}", result, now.elapsed().as_millis());

    let now = std::time::Instant::now();
    let result = fastest(bytes, count);
    println!("fastest({:?}): {}", result, now.elapsed().as_millis());

    let now = std::time::Instant::now();
    let result = fastester(bytes, count, 0, bytes.len());
    println!("fastester({:?}): {}", result, now.elapsed().as_millis());

    let now = std::time::Instant::now();
    let result = fastester_async(string, count);
    println!("fastester_async({:?}): {}", result, now.elapsed().as_millis());

    let now = std::time::Instant::now();
    let result = fastesterest_sync(bytes, count, 12);
    println!("fastesterest_sync({:?}): {}", result, now.elapsed().as_millis());

    let now = std::time::Instant::now();
    let result = fastest_async(bytes, count, 12);
    println!("fastest_async({:?}): {}", result, now.elapsed().as_millis());

    let now = std::time::Instant::now();
    let result = fastesterest_async(bytes, count, 12);
    println!("fastesterest_async({:?}): {}", result, now.elapsed().as_millis());

    /*

    let now = std::time::Instant::now();
    let result = fastest(bytes, count);
    println!("fastest({}): {}", result, now.elapsed().as_millis());

    let now = std::time::Instant::now();
    let result = fastester(bytes, count, 0, bytes.len());
    println!("fastester({:?}): {}", result, now.elapsed().as_millis());

    let now = std::time::Instant::now();
    let result = fastesterest_async(bytes, count, 10);
    println!("fastesterest_async({:?}): {}", result, now.elapsed().as_millis());

    let now = std::time::Instant::now();
    let result = fastester_async(string, count);
    println!("fastester_async({:?}): {}", result, now.elapsed().as_millis());

    let now = std::time::Instant::now();
    let result = fastester_async(string, count);
    println!("fastester_async({:?}): {}", result, now.elapsed().as_millis());
    */

    /*
    let mut result = simple(string, count);
    println!("simple({}): {}", result, now.elapsed().as_millis());

    let now = std::time::Instant::now();
    result = faster(string, count);
    println!("faster({}): {}", result, now.elapsed().as_millis());

    let now = std::time::Instant::now();
    let cpus = 12;
    let regions = string.len() / cpus;
    let mut handles: Vec<JoinHandle<Option<usize>>> = vec![];
    for i in 0..cpus {
        let start = if i == 0 { 0 } else { i * regions - count + 1 };
        let mut len = regions;

        if i == regions - 1 {
            len = string.len();
        }
        handles.push(std::thread::spawn(move || {
            if let Some(x) = fastest(bytes, count, start, len) {
                return Some(x + start);
            }
            return None;
        }));
    }

    let mut answer = 0;
    for handle in handles {
        let found = handle.join();
        if let Ok(Some(a)) = found {
            answer = a;
            break;
        }
    }

    println!("fastest_multi({:?}): {}", answer, now.elapsed().as_millis());
    */

    //println!("final answer: {}", &string[result - count..result])
}


#[cfg(test)]
mod test {
    use crate::{fastester, fastest, PROD};

    #[test]
    fn test_cond_1() {
        let str = "beginxxxxxxx";
        let result = fastester(str.as_bytes(), 5, 0, str.len());

        assert_eq!(Some(5), result);
    }

    #[test]
    fn test_cond_2() {
        let str = "xxxxxbeginxxxxxxx";
        let result = fastester(str.as_bytes(), 5, 0, str.len());

        assert_eq!(Some(10), result);
    }

    #[test]
    fn test_cond_split_even() {
        let str = "xxxbeginxxxxxxx";
        let result = fastester(str.as_bytes(), 6, 0, str.len());

        assert_eq!(Some(9), result);
    }

    #[test]
    fn test_cond_split_even_with_extra() {
        let str = "xxybeginxxxxxxx";
        let result = fastester(str.as_bytes(), 6, 0, str.len());
        assert_eq!(Some(9), result);
    }

    #[test]
    fn test_cond_split_odd_left_last() {
        let str = "xxybeginxxxxxxx";
        let result = fastester(str.as_bytes(), 7, 0, str.len());
        assert_eq!(Some(9), result);
    }

    #[test]
    fn test_cond_split_odd_right_last() {
        let str = "xxxxxxxxybeginxxxxxxx";
        let result = fastester(str.as_bytes(), 7, 0, str.len());
        assert_eq!(Some(14), result);
    }

    #[test]
    fn test_all() {
        let f = fastest(PROD.as_bytes(), 14);
        let fer = fastester(PROD.as_bytes(), 14, 0, PROD.len());
        assert_eq!(f, fer.unwrap());
    }

    #[test]
    fn test_big() {
        let string = std::fs::read_to_string("long").expect("please generate the file first, script in src dir");
        let string: &'static String = Box::leak(Box::new(string));

        let f = fastest(string.as_bytes(), 14);
        let fer = fastester(string.as_bytes(), 14, 0, string.len());
        assert_eq!(f, fer.unwrap());
    }
}
