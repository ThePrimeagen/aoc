let file = "d1p1"

let is_digit ch =
    match ch with | '0'..'9' -> true | _ -> false

let parse_digit ch =
    int_of_char ch - int_of_char '0'

let min_position a b =
    match a, b with
    | (-1, _), (_, _) -> b
    | (_, _), (-1, _) -> a
    | (a_index, _), (b_index, _) when a_index < b_index -> a
    | _ -> b

let max_position a b =
    match a, b with
    | (-1, _), (_, _) -> b
    | (_, _), (-1, _) -> a
    | (a_index, _), (b_index, _) when a_index > b_index -> a
    | _ -> b


let is_named_int str =
    match str with
    | s when String.starts_with s ~prefix:"one" -> 1
    | s when String.starts_with s ~prefix:"two" -> 2
    | s when String.starts_with s ~prefix:"three" -> 3
    | s when String.starts_with s ~prefix:"four" -> 4
    | s when String.starts_with s ~prefix:"five" -> 5
    | s when String.starts_with s ~prefix:"six" -> 6
    | s when String.starts_with s ~prefix:"seven" -> 7
    | s when String.starts_with s ~prefix:"eight" -> 8
    | s when String.starts_with s ~prefix:"nine" -> 9
    | _ -> -1

let find_first_named_int str =
    let rec aux i =
        let end_of_string = min ((String.length str) - i) 5 in
        if i >= String.length str then
            (-1, 0)
        else if is_named_int (String.sub str i end_of_string) > 0 then
            (i, is_named_int (String.sub str i end_of_string))
        else
            aux (i + 1)
    in
    aux 0

let find_last_named_int str =
    let rec aux i =
        let len = min ((String.length str) - i) 5 in
        if i < 0 then
            (-1, 0)
        else if is_named_int (String.sub str i len) > 0 then
            (i, is_named_int (String.sub str i len))
        else
            aux (i - 1)
    in
    aux ((String.length str) - 1)

let find_first_digit str =
  let rec aux i =
    if i >= String.length str then
        (-1, 0)
    else if is_digit str.[i] then
        (i, parse_digit str.[i])
    else
      aux (i + 1)
  in
  aux 0

let find_last_digit str =
  let rec aux i =
    if i < 0 then
        (-1, 0)
    else if is_digit str.[i] then
        (i, parse_digit str.[i])
    else
      aux (i - 1)
  in
  aux ((String.length str) - 1)


let rec solve_day_1_part_1 channel sum =
    match input_line channel with
    | line ->
        let (_, first_digit) = min_position (find_first_digit line) (find_first_named_int line) in
        let (_, last_digit) = max_position (find_last_digit line) (find_last_named_int line) in

        print_endline Format.(sprintf "line: %s first_digit: %d, last_digit: %d" line first_digit last_digit);
        solve_day_1_part_1 channel (sum + (first_digit * 10) + last_digit)
    | exception End_of_file -> sum

let () =
    (* Read file and display the first line *)
    let ic = open_in file in
    print_endline (Int.to_string (solve_day_1_part_1 ic 0))
