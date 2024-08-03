use std::{collections::HashSet, sync::mpsc::Receiver};

use criterion::black_box;

// const PROD: &'static str = "qvllndllhzhfzhhdzhddhjdjggvnvhvccmffwllqgqmmfjfqfhhtrrzczjczzlplddfpptqqfbqffmnmjnnqppfjfccgnnmqqsvvdbbgppjvpjvpjjctjjttwtrrdldlcddrvddqndqnqwqwzwfwwzczggcppgzpzhpzhppprfffbhhwmhhtftstrsrvsrvsrvvshvssnwwpllhfhnnfflcltlblzlqlvqlvlcldcccpptggtdgdjdbbrggmbmnncscbssqrrjddvcvgvfflpppgpvphvphhpcpzpzvvctvctvthtwtfwwbrrhhlplmlwwlqlnlhhtmhmmqlqplllrvrgvrvrffzfgfjfjtjmjvmjmwmvvjffmpfphfhvfvmfvmmhpphhltthgttgccqggpzpfpqpcpvcpvcvvqtqvqbbrlrtllmrmllhmhvmhhvzhvzvrrrzjzbbtvvbgvbbfnnqndqnnpnbnbnlnggwqggmgmqmgmbbmccgqcqbccpvcvnnhvvrvlrrcwrcwrcwrwbrwwzbwbdbfddpttntzzjszsnznbndnzngzgccjrcjchcffmlmqqlrqqzsqzzsbsnsttzpztpzpggzrrttbqqplpqlqjjqcqvccdzdccthccvfcvvqvhqhfhhzwzpzwppgpttntssflfjjrwrqrjrppptlltptpvttpfpwpswpppzzsrzssqllbnlljpllrjllsrlrhrdrmdrmrrpsrprnrffgrffdqdhdqhhrhggwqqlddsbsqbqtqdtdhdvhhbdhdzhdhhtrrppzddgfgzgpzpvpfpnpptggltggbnbppqffzfrzzzsbsrrdgrddwsdsqddhpdpbpvpfvppfsfgfngffzmzbzlblclsccvqvqmmjtjqtjjlcjllsddjqddhldlvlrrbgbrgbrrdzzpfpggqnqbqrrqbbgjgppqgpgwgqqndncndnpdnnbvbnvnwnjjgppzlplqqdgqghqgqzggjssqmmwwcfcpptrpprggrppgbplmzwmdtnpqwzcrthqbppwbgcvgqrpfpnbscnhvrllpvpqwnsslcjrqtvdccprvqfrpswtpvzdzlgtmmvppdmhgdbbsmrbqpqspdhpqgfjznqzphrnggcbzhdqrgvzcfzrhtrlssgmjjghqsjtghhnwjffqrrfslfnsvvdvfjqbfpffrrstdhggvbfwtfpfgswqlfdrnjpjmwzptlbmwgghgwqrphcrvfmhrplllgbnjlprllmjwccphsflntgpnbmdbfqcdsbgvrnfznfrlcfvswqfrqvdnbjsflnsmlcrdstzppmcvbgdtcvgztbdzqbwhmwcfvbwjjcdgbnwjwzrrdqhpgscwtnztjsfstzfwftcldjgvdvwbzrlbdslwttbqpnlwbjcjwqgtrgcglsgtdqbqbnqznptzzbwffwlwzvvtdpcjbvhnswzptclpbndcdvsfmcrmwwgzdfsszqjjdztmtsqgfqzjpctfdpwnzbpnzzwngqnghntblndfrnjzdrmgbqmzbdqfzctrgshwqgfgqssqjltrqlzjswjhmpgwwjdwcjpnsvgrvbfpmlmmwzmbdjwsrjthppfrccjgnmwlvqlprgslbwtbbzlqbznczmsmhsfdcqnwblprcpbzzwfllbnldvpjcwsdhglrzjsptmsjdjqzsmgvhjfjrrtvvbjlmzjsntnrggwbpjlrjggfgqzvswtggthzfmfjnmrzrttbzqpwpsnmdtnbfblpfgslgcmjlbdpshnnrbhvwsbrnvdmjqhvhdjhbfzjmqrmqmdthhzvnrmqcnbtwcdjdqfvdgvmfbhrfqnmdncrddggtcppjlznbsnntppjtnsqsrjwvfrzpnzqcrzhhdflfmmtmwcvtpzbqhdwsczffcqhtdbdjblmgnrmhlqcsvcpgghhvwqhdtzpzlpfllchzltqgcwgfqnbzhgzmdwqdlwnvhqmpqjqnjbhjctslghdqvctdmjfwdfpdjnhdndzwsfjzlmsbmfmzvnvpqgqhtngvgqmlrrzsfmwlcwsscvghjvrzjjqbnplnjzqswpblwzwczhwbhhnjmctnmwlbqqfmnlwdcrptlmfjpjrnpcvmhffjhwhmntdzpdjzwzhrrsdvmjlwdtcpvjfmfzfsrgjghhlvmjjjczgmhvrfpgqbnhldwbrjgzmnszzbssfzcggrwmdfvddwsdmnwtwfwlfnwlvzlctfblbtrjvcwjjdljplcrjhwqslppwwtvfqwsjlfmdznmcdzdmgvmmsrfcclcvhtrhlsjzrbjwrjlfnvqhqvmpzmdttnbhfcvnqlrqbcsvtvwfccjstjpmhqgwlnrzjjmfdszflmglrdbpqhqhqsdfzrcljbdvvnlcqfllmnqcjfzjppdsjwshfschzqbnwfqnpwhqnmwsjbtcgvrljsrtzvcvghcjjlqsngglcggqpntrrhbjpbfhmvpltmnfmfdtwnczwfbvjcqnhvppjftwvwsrlhvvcjtsfptpqgrmrqwwddnqmnmfgrlnphbpqhhhvglqgtwvnwvnbssftmwttmfrffwtzhrpqspclvgchwqwcsgwqwwvpgcwngrcfmhbhflwfbfchlphdzdcrflfmfclsngtlwrqcrsgrdzcpdsvvcdbhgtljmbntbbcqgjqfsbfwzlfsnljpjdcnmjlqrwpmlvwgdlrrdgfhdqhzgltmclzgzzhmrbggsmgtpqdrgmjtlzwstrwbpvhppvsmdqvvwwglzjgdswjszqmrdbmshbhhcstpcsjdbvgjnvcmvhbtclrlmlgnvppgvncsrfchdbqjrclwwlnchmcgvshfsbsvvcvjrsgjlnsfqtqmgntffwnqjtldcqbcqhsgztllstswwqnfrswpchqhnfzzzszqjztzfrgrbjdbjlpvqfqrlrmmpbfbbcclrgmnlzwqrjhqrstswjpgsrtnlwsbqthzpvdzllzqmdmbvvtcztftvlwphhjzbfnrvccfmhmvmzlbrzlnppfzcsffjvjmbgpvlwgwszpztjpsrbnftqtdrbnljtbrjzzbwlsvtwtlwptdtnmtncvcblcmdngjzmctlqtzchncccnwjzrrmmmnllbhrnhwtqjsnvcslrqjfbfndqvdlrjshdzmlprtzbtnhthdqhplwzdbnjmgzlzrbzrvrqnflwfmsmbssqnbcddnvdpltpmplpdzvtjrslcdcnrdplwtjtvctwfzhlvwwqqtbqcjjwhhnpmvgzhqmqfgthwbphrmrtdghchsmwghdqjgjgmpddbrtngtvhqgjfrplrdgpbnhqvswrmqhcmsqvsqmqsgwjndwjrbrhvrctmmrmfwpsgfgdlrzpslpflgvwrgcthgcrnhgrzsmqdgdssjgspfhmqfmjfpmwqhnfjdvqzhpndvnbmqglbrjmdrwgmgctrgzpsdvfbmcstcslblmvnprphntgslmlrqwthrndrhtbccgzzfsglhgqztcsnqjwfzbzlvrpbvswbhrwdsrhrrpnrmsbvbvjccbdsdcfrrzpgwjtnnnvjwlcppwzdqsbdzpfjplrlfgvjpsmbzwpwlghnvqgddfjvrsztrpzlfgmqqzrfcgglghndbhgbmldglclhldljjdslvhzshshtqwhqnbzhvqrcmwdmcmhjcrmdmhrwnwcbhvbbrwrbtfdnztwnbpdfjfhgrmcpngftsvbsmsptnwcvvllnmbnsntbzmwnhfdptbtzswtjzdqwjdhprnjwvhzpscjvlsgrhdrmmrmhzhwwtslzdjqmzfncnmgplhnmwrvqhslvchtjcmpzpjpnpfbjptvvwcsmhgdjtsqrjlfpnfdncpqqmpgpvtlvwljlsqbnhtsqgfwlsmdjpgtvgjvjcrnnzmbllqzlrfdnlffgmtphhhgbcjgdlpzqpwmjwtcmdrsmtnmddftwczbsddtppsptbwfvpnfnsqmsgcfqfmnzffzqgcdvwzrgdwhmnzmrlhcdpdsltnsmjzdqwmmpwvjqbbwsrfgzh";

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

/*
fn to_string(data: &[u8], start: usize, stop: usize) -> String {
    return data[start..=stop].iter().filter_map(|x| char::from_u32(*x as u32)).collect::<String>();
}
*/

fn simple(i: &[u8]) -> usize {
    return i.windows(14)
        .position(|w| {
            return w.iter().collect::<HashSet<_>>().len() == 14;
        })
        .map(|x| x + 14)
        .unwrap();
}

fn faster_vec(i: &[u8]) -> usize {
    return i.windows(14)
        .position(|w| {
            let mut vec = Vec::with_capacity(14);
            for x in w {
                if vec.contains(x) {
                    return false;
                }

                vec.push(*x);
            }
            return true;
        })
        .map(|x| x + 14)
        .unwrap();
}

fn faster_arr(i: &[u8]) -> usize {
    return i.windows(14)
        .position(|w| {
            let mut arr = [0u8; 14];
            let mut idx = 0;
            for x in w {
                for i in 0..idx {
                    if arr[i] == *x {
                        return false;
                    }
                }
                arr[idx] = *x;
                idx += 1;
            }
            return true;
        })
        .map(|x| x + 14)
        .unwrap();
}

fn faster(i: &[u8]) -> usize {
    return i.windows(14)
        .position(|w| {
            let mut hash_set = HashSet::new();
            for x in w {
                if !hash_set.insert(x) {
                    return false;
                }
            }
            return true;
        })
        .map(|x| x + 14)
        .unwrap();
}

fn david_a_perez_async(data: &'static [u8], cpus: usize) -> Option<usize> {
    let regions = data.len() / cpus;
    let (tx, rx) = std::sync::mpsc::channel();
    for i in 0..cpus {
        let start = if i == 0 { 0 } else {i * regions - 14};
        let inner_tx = tx.clone();
        let mut len = regions;

        if i == regions - 1 {
            len = data.len();
        }

        std::thread::spawn(move || {
            if let Some(x) = david_a_perez(&data[start..start + len]) {
                _ = inner_tx.send(Some(x + start));
            } else {
                _ = inner_tx.send(None);
            }
        });
    }

    return join(rx, cpus);
}

pub fn benny(input: &[u8]) -> Option<usize> {
    let mut filter = 0u32;
    input
        .iter()
        .take(14 - 1)
        .for_each(|c| filter ^= 1 << (c % 32));

    input.windows(14).position(|w| {
        let first = w[0];
        let last = w[w.len() - 1];
        filter ^= 1 << (last % 32);
        let res = filter.count_ones() == 14 as u32;
        filter ^= 1 << (first % 32);
        res
    })
}

struct ReverseIter {
    slice: &'static [u8],
    pos: isize
}

impl Iterator for ReverseIter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < 0 {
            return None;
        }
        let out = self.slice[self.pos as usize];
        self.pos -= 1;
        return Some(out);
    }
}

fn reverse_iterator(item: &'static [u8]) -> ReverseIter {
    return ReverseIter {
        slice: item,
        pos: (item.len() - 1) as isize,
    }
}

pub fn david_a_perez_2(input: &'static [u8]) -> Option<usize> {
    let mut idx = 0;
    while let Some(slice) = input.get(idx..idx + 14) {
        let mut state = 0u32;

        if let Some(pos) = reverse_iterator(slice).position(|byte| {
            let bit_idx = byte % 32;
            let ret = state & (1 << bit_idx) != 0;
            state |= 1 << bit_idx;
            ret
        }) {
            idx += 13 - pos + 1;
        } else if state.count_ones() == 14 as u32 {
            return Some(idx);
        }
    }
    return None;
}

pub fn david_a_perez(input: &[u8]) -> Option<usize> {
    let mut idx = 0;
    while let Some(slice) = input.get(idx..idx + 14) {
        let mut state = 0u32;

        if let Some(pos) = slice.iter().rposition(|byte| {
            let bit_idx = byte % 32;
            let ret = state & (1 << bit_idx) != 0;
            state |= 1 << bit_idx;
            ret
        }) {
            idx += pos + 1;
        } else if state.count_ones() == 14 as u32 {
            return Some(idx);
        }
    }
    return None;
}

pub fn mod_andreselcientifico_david_a_perez(input: &[u8]) -> Option<usize> {
    let mut idx = 0;
    while let Some(slice) = input.get(idx..idx + 14) {
        let mut state = 0u32;

        if let Some(pos) = slice.iter().rposition(|byte| {
            let bit_idx = byte % 32;
            state |= 1 << bit_idx;
            state & (1 << bit_idx) != 0
        }) {
            idx += pos + 1;
        } else if state.count_ones() == 14 as u32 {
            return Some(idx);
        }
    }
    return None;
}

pub fn david_a_perez_the_primeagen(input: &[u8]) -> Option<usize> {
    let mut idx = 14 - 1;
    while let Some(slice) = input.get(idx - 13..idx + 14) {

        let mut left_state = 0u32;
        let left = slice[0..14].iter().rposition(|byte| {
            let bit_idx = 1 << (byte % 32);

            let ret = left_state & bit_idx != 0;
            if !ret {
                left_state |= bit_idx;
            }
            return ret;
        });

        let mut right_state = 0u32;
        let right = slice[14..27].iter().rposition(|byte| {
            let bit_idx = 1 << (byte % 32);

            let ret = right_state & bit_idx != 0;
            if !ret {
                right_state |= bit_idx;
            }
            return ret;
        });

        if let None = left {
            return Some(idx + 1);
        }

        if let None = right {
            return Some(idx + 14);
        }

        let right = right.unwrap();
        let left = left.unwrap();
        let diff = right - left;

        if diff >= 14 {
            let unique_count = (left_state ^ right_state).count_ones();
            let unique_count = (diff - unique_count as usize) / 2 + unique_count as usize;
            if unique_count >= 14 {
                if let Some(x) = benny(&input[idx - left..idx + right]) {
                    return Some(x);
                }
            }
        }

        idx += right + 14;
    }
    return None;
}

pub fn david_a_perez_proc(input: &[u8]) -> Option<usize> {
    let mut idx = 0;
    while let Some(slice) = input.get(idx..idx + 14) {
        let mut state = 0u32;
        let mut pos = (slice.len() - 1) as isize;
        while pos >= 0 {
            let bit_idx = 1 << (slice[pos as usize] % 32);
            if state & bit_idx != 0 {
                break;
            } else {
                state |= bit_idx;
                pos -= 1;
            }
        }

    if pos < 0 {
            return Some(idx);
        }
        idx += (pos + 1) as usize;
    }
    return None;
}

fn main() {

    let string = std::fs::read_to_string("long").unwrap();
    let string: &'static String = Box::leak(Box::new(string));
    // let string = PROD;
    let bytes = string.as_bytes();

    let mut vec = vec![];

    for _ in 0..10 {
        let now = std::time::Instant::now();
        black_box(simple(black_box(bytes)));
        vec.push((now.elapsed(), "simple"));
    }

    for _ in 0..10 {
        let now = std::time::Instant::now();
        black_box(faster(black_box(bytes)));
        vec.push((now.elapsed(), "faster"));
    }

    for _ in 0..10 {
        let now = std::time::Instant::now();
        black_box(faster_vec(black_box(bytes)));
        vec.push((now.elapsed(), "faster_vec"));
    }

    for _ in 0..10 {
        let now = std::time::Instant::now();
        black_box(faster_arr(black_box(bytes)));
        vec.push((now.elapsed(), "faster_arr"));
    }

    for _ in 0..10 {
        let now = std::time::Instant::now();
        black_box(benny(black_box(bytes)));
        vec.push((now.elapsed(), "benny"));
    }

    for _ in 0..10 {
        let now = std::time::Instant::now();
        black_box(david_a_perez(black_box(bytes)));
        vec.push((now.elapsed(), "david_a_perez"));
    }

    for _ in 0..10 {
        let now = std::time::Instant::now();
        black_box(mod_andreselcientifico_david_a_perez(black_box(bytes)));
        vec.push((now.elapsed(), "mod_andreselcientifico_david_a_perez"));
    }

    for _ in 0..10 {
        let now = std::time::Instant::now();
        black_box(david_a_perez_async(black_box(bytes), 18));
        vec.push((now.elapsed(), "dap_async"));
    }

    println!("{}", vec.iter().map(|x| format!("\n{} {:?}", x.1, x.0)).collect::<String>());
}

