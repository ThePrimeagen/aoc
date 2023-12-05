let name = "d3"

let max_int a b =
    if a > b then a
    else b

let is_dot c =
    c = '.'

let is_number c =
    c >= '0' && c <= '9'

let is_symbol c =
    let dot = is_dot c in
    let num = is_number c in
    not dot && not num

let is_star c =
    c = '*'

let rec repeat n s =
    if n = 0 then ""
    else s ^ repeat (n - 1) s

let normalize_str s max =
    if String.length s < max then
        "." ^ s ^ repeat (max - String.length s) "." ^ "."
    else
        "." ^ s ^ "."

let rec get_max_len l =
    match l with
    | [] -> 0
    | h :: t -> max_int (String.length h) (get_max_len t)

let rec inner_normalize_list l max_len =
    match l with
    | [] -> []
    | h :: t -> normalize_str h max_len :: inner_normalize_list t max_len

let create_full_row_empty max_len =
    [repeat (max_len + 2) "."]

let normalize_list l max_len =
    create_full_row_empty max_len @ inner_normalize_list l max_len @ create_full_row_empty max_len

let has_symbol str =
    let rec aux str idx =
        if String.length str = idx then
            false
        else if is_symbol str.[idx] then
            true
        else
            aux str (idx + 1)
    in
    aux str 0

let find_next_int str idx s l =
    let rec aux str idx in_number s l =
        if String.length str = idx then
            if in_number then
                idx, s, Some l
            else
                idx, None, None
        else if is_number str.[idx] then
            match s with
            | None -> aux str (idx + 1) true (Some idx) 1
            | _ -> aux str (idx + 1) true s (l + 1)
        else if in_number then
            idx + 1, s, Some l
        else
            aux str (idx + 1) false s l
    in

    aux str idx false s l

let rec get_num_list str idx l =
    match find_next_int str idx None 0 with
    | _, None, None -> l
    | next_idx, Some s, Some len -> get_num_list str next_idx ((s, len) :: l)
    | _ -> failwith "Invalid input"

let rec read_all ic lines =
    match In_channel.input_line ic with
    | None -> lines
    | Some line -> read_all ic (line :: lines)
    | exception End_of_file -> lines

let rec day3_part1 lines sum =
    match lines with
    | p :: c :: n :: tail ->
        let list_of_numbers = get_num_list c 0 [] in
        let value = List.fold_left (fun acc (s, l) ->
            let ext_s = s - 1 in
            let ext_l = l + 2 in
            let value = (String.sub c s l |> int_of_string) in
            if String.sub p ext_s ext_l |> has_symbol then
                acc + value
            else if String.sub c ext_s ext_l |> has_symbol then
                acc + value
            else if String.sub n ext_s ext_l |> has_symbol then
                acc + value
            else
                acc
        ) 0 list_of_numbers in
        day3_part1 (c :: n :: tail) (sum + value)
    | _ -> sum

let row_projection = 1000000
let rec projection_list lines list_idx out_lines =
    match lines with
    | _ :: c :: n :: tail ->
        let list_of_numbers = get_num_list c 0 [] in
        let list_of_numbers = List.fold_left (fun acc (s, l) ->
            let value = (String.sub c s l |> int_of_string) in
            let proj = list_idx * row_projection in
            (proj + s, proj + s + l - 1, value) :: acc
        ) [] list_of_numbers in
        projection_list (c :: n :: tail) (list_idx + 1) (out_lines @ list_of_numbers)
    | _ -> out_lines

let rec star_positions line line_idx idx pos =
    if String.length line = idx then
        pos
    else if is_star line.[idx] then
        star_positions line line_idx (idx + 1) ((line_idx * row_projection + idx) :: pos)
    else
        star_positions line line_idx (idx + 1) pos

let rec star_list lines idx out_lines =
    match lines with
    | h :: t -> star_list t (idx + 1) ((star_positions h idx 0 []) @ out_lines)
    | _ -> out_lines

let get_intersections projections s e =
    List.fold_left (fun acc (s', e', v) ->
        if s <= e' && e >= s' then
            v :: acc
        else if s <= s' && e >= s' then
            v :: acc
        else
            acc
    ) [] projections

let print_projections projections =
    Printf.printf "Projections:\n";
    List.iter (fun (s, e, v) ->
        Printf.printf "   %d %d %d\n" s e v
    ) projections

let print_int_list name stars =
    Printf.printf "%s:\n" name;
    List.iter (fun s ->
        Printf.printf "   %d\n" s
    ) stars

let rec day3_part2 projections stars sum =
    match stars with
    | h :: tail ->
        let right = h - 1 in
        let left = h + 1 in
        let intersections =
            get_intersections projections (right - row_projection) (left - row_projection) @
            get_intersections projections right left @
            get_intersections projections (right + row_projection) (left + row_projection) in
        begin

        print_endline "-------------------";
        print_endline ("Star " ^ string_of_int h);
        print_int_list "Intersections" intersections;

        match intersections with
        | [x ; y] -> day3_part2 projections tail sum + x * y
        | _ -> day3_part2 projections tail sum
        end
    | _ -> sum

let () =
    let contents = open_in name in
    let contents = read_all contents [] |> List.rev in
    let max_len = get_max_len contents in
    let contents = normalize_list contents max_len in
    print_endline (string_of_int (day3_part1 contents 0));

    let projections = projection_list contents 1 [] in
    let stars = star_list contents 0 [] in
    print_projections projections;
    print_int_list "Stars" stars;
    print_endline (string_of_int (day3_part2 projections stars 0));
    ()

