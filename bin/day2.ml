let file = "d2"

let max_int a b =
    if a > b then
        a
    else
        b

(*

let red_max = 12
let green_max = 13
let blue_max = 14
let exceeds_max name count =
    match name with
    | "red" -> count > red_max
    | "blue" -> count > blue_max
    | "green" -> count > green_max
    | _ -> false

let valid_round round =
    let rec aux rounds =
        match rounds with
        |  h :: t ->
            begin
            match String.trim h |> String.split_on_char ' ' with
            | [count ; color] ->
                let invalid = exceeds_max color (int_of_string count) in
                if invalid then
                    false
                else
                    aux t
            | _ -> false
            | exception Not_found -> false
            end
        | [] -> true
    in

    match (Base.String.split round ~on:',') with
    | rounds -> aux rounds
    | exception Not_found -> aux [round]
    *)

let max_set (a, b, c) (x, y, z) =
    (max_int a x, max_int b y, max_int c z)

let min_dice round =
    let rec aux rounds red green blue =
        match rounds with
        |  h :: t ->
            begin
            match String.trim h |> String.split_on_char ' ' with
            | [count ; color] ->
                begin
                match color with
                | "red" -> aux t (max_int red (int_of_string count)) green blue
                | "green" -> aux t red (max_int green (int_of_string count)) blue
                | "blue" -> aux t red green (max_int blue (int_of_string count))
                | _ -> red, green, blue
                end
            | _ -> red, green, blue
            | exception Not_found -> red, green, blue
            end
        | [] -> red, green, blue

    in

    match (Base.String.split round ~on:',') with
    | rounds -> aux rounds 0 0 0
    | exception Not_found -> aux [round] 0 0 0

(*
let print_list list =
    List.iter (fun x -> print_endline @@ "    " ^ x) list


let parse_game_id line =
    let start = (String.index line ' ') + 1 in
    let end_len = String.length line - start in
    let game_id = String.sub line start end_len in
    print_endline ("\"" ^ game_id ^ "\"");
    int_of_string game_id
*)

let parse_rounds line =
    let rec aux line list =
        let end_idx = match (String.index line ';') with
        | end_idx -> end_idx
        | exception Not_found -> String.length line in

        let round = String.sub line 0 end_idx in

        let next_start = end_idx + 2 in
        if next_start >= String.length line then
            round :: list
        else
            aux (String.sub line next_start (String.length line - next_start)) (round :: list)
    in

    aux line []

let rec solve_day_2_part_2 channel sum =
    match input_line channel with
    | line ->
        begin
        match String.split_on_char ':' line with
        | _ :: rounds :: _ ->
            let rounds = parse_rounds @@ String.sub rounds 1 (String.length rounds - 1) |>
            List.map (fun x -> min_dice x) in
            let (a, b, c) = List.fold_left (fun acc x -> max_set acc x) (0, 0, 0) rounds in
            let sum = sum + (a * b * c) in

            print_endline ("Current Sum: " ^ string_of_int sum ^ " " ^ string_of_int a ^ " " ^ string_of_int b ^ " " ^ string_of_int c);
            print_endline line;
            solve_day_2_part_2 channel sum
        | _ -> failwith "Invalid input";
        end
    | exception End_of_file -> sum

(*
let rec solve_day_2_part_1 channel sum =
    match input_line channel with
    | line ->
        begin
        match String.split_on_char ':' line with
        | game_id :: rounds :: _ ->
            let game_id = parse_game_id game_id in
            let rounds = parse_rounds @@ String.sub rounds 1 (String.length rounds - 1) |>
            List.map (fun x -> valid_round x) in

            let valid = List.fold_left (fun acc x -> if acc && x then true else false) true rounds in
            let sum = if valid then sum + game_id else sum in

            print_endline ("Current Sum: " ^ string_of_int sum);
            print_endline line;
            solve_day_2_part_1 channel sum
        | _ -> failwith "Invalid input";
        end
    | exception End_of_file -> sum

*)
let () =
    let channel = open_in file in
    let _ = solve_day_2_part_2 channel 0 in
    ()

