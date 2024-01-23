let rec power x n =
  match n with
    0 -> 1
  |_ -> x * power x (n - 1)


let rec somatorial n =
  match n with
    0 -> 0
  |_ -> n + (somatorial (n-1))


let isvowel letter = 
  match letter with
    'a' | 'e' | 'i' | 'o' | 'u' -> true
  | 'A'| 'E' | 'I' | 'O' | 'U' -> true
  | _ -> false


let isconsonant letter =
  match letter with
    'a'..'z' -> true && not (isvowel letter)
  | 'A'..'Z' -> true && not (isvowel letter)
  |  _ -> false


let islower x =
  match x with
    'a'..'z' -> true
  | _ -> false


let isupper x = 
  match x with
    'A'..'Z' -> true
  | _ -> false


let rev l =
  let rec aux list buffer = 
    match list with
      h::t -> aux t (h::buffer)
    | _ -> buffer
  in aux l []


let rec even_idx l = 
  match l with
    _::b::t -> b::even_idx t
  | _ -> []


let count_true l =
  let rec aux l n =
    match l with
      h::t -> if h then 1 + aux t n else aux t n
    |_ -> n
  in aux l 0


let rec drop n l =
  if n=0 then l else
    match l with
      h::t -> drop (n-1) t
    |_ -> l 


let rec palin1 l =
  match l with
    [a] -> [a]
  |h::t -> h::(palin1 t @ [h])
  | _ -> l


let palin2 l = l @ drop 1 (rev l)

let rec drop_last l =
  match l with
    [a] -> []
  | h::t -> h::drop_last t
  | _ -> []


let rec last l = 
  match l with
    [a] -> a
  | h::t -> last t


let rec is_palin l = 
  match l with
    [] -> true
  | [a] -> true
  | h::t -> h = (last t) && is_palin (drop_last t)


let rec member_rec x l =
  match l with
    [] -> false
  | h::t -> h=x || member_rec x t  
 

let rec filter f l =
  match l with
    [] -> []
  | h::t -> if f h then h:: filter f t else filter f t


let rec qs l =
  match l with
    [] -> []
  | h::t -> qs (filter (fun x -> x<h) t) @ h :: qs (filter (fun x -> x>=h) t)


let rec isort l =
  match l with
    [] -> []
  | h::t ->
    let rec insert x l =
      match l with
        [] -> [x]
      | h::t -> if h>=x then x::l else h::(insert x t)
    in insert h (isort t)


let rec merge lx ly =
  match lx, ly with
    [], l -> l
  | l, [] -> l 
  | hx::tx, hy::ty ->
    if hx < hy
    then hx::merge tx ly
    else hy::merge lx ty


let take n l = 
  let rec aux n l buffer =
    match n, l with
      0, _ -> buffer
    | _, [] -> buffer 
    | _, h::t -> aux (n-1) t (h::buffer)
  in aux n l []


let length l = 
  let rec aux l buffer =
    match l with
      [] -> buffer
    | h::t -> aux t buffer + 1 
  in aux l 0


let rec msort l = 
    match l with
        [] -> []
      | [a] -> [a]
      | _ -> 
        let len = length l / 2 in
          let left = take len l in
            let right = drop len l in
        merge (msort left) (msort right)


let rec issorted l = 
  match l with
    [] -> true
  | [a] -> true
  | h::x::t -> h<=x && issorted (x::t)


let rec calm_rec chars =
  match chars with
    [] -> []
    | h::t ->
        if h = '!'
        then '.':: calm_rec t
        else h::calm_rec t

let rec map f l =
  match l with
    [] -> []
    | h::t -> f h :: map f t

let calm chars = map (fun c -> if c = '!' then '.' else c) chars

let clip i =
  match true with
    over when over = (i>10) -> 10
  | under when under = (i<1) -> 1
  | _ -> i

let clip_list l = map clip l

let rec apply f n arg =
  match n with
    1 -> f arg
  | 0 -> arg
  | _ -> f (apply f (n-1) arg)



let rec for_all_rec f l = 
  match l with
    [] -> false
  | [a] -> f a
  | h::t -> (f h) && for_all_rec f t


let rec reduce f l =
  match l with
    [a] -> a
  | h::s::t -> reduce f ((f h s)::t)

(*let all l = reduce (fun x y -> x && y) l*)
let all l = reduce (&&) l

let for_all f l = all (map f l)

let rec mapl f l =
  match l with
    [] -> []
  | h::t -> (map f h)::mapl f t       

let rec range i j =
  if i=j then [i] else i::(range (i+1) j)

let any l = reduce (||) l

let member x l = any (map (fun v -> x=v) l) (*definitivamente a versão recursiva tem melhor desempenho*)

let min x y = if x<y then x else y
let max x y = if x<y then y else x

(*exception Not_Found*)

let head l = 
  match l with
    [] -> raise Not_found
  | h::t -> h

let smallest l =
  let rec aux x l =
    match l with
      [] -> x
    | h::t -> aux (min x h) t
  in aux (head l) l


let rec smallest_pos l =
  let rec aux x l =
    match l with
      [] -> if x>=0 then x else raise Not_found
    | h::t -> 
      if h>=0 && x>=0 then 
        aux (min h x) t 
      else 
      if h>=0 then 
        aux h t 
      else 
      if x>=0 then 
        aux x t 
      else smallest_pos t
  in aux (head l) l 

let smallest_or_zero l =
  try smallest_pos l with Not_found -> 0



let floor_sqrt x = 
  if x<0 then 
    invalid_arg "x must be positive"
  else
  let rec aux num cur_guess past_guess =
    let sq_guess = cur_guess*cur_guess in
      if sq_guess=num then
        cur_guess
      else
      if sq_guess<num && ((cur_guess+1)*(cur_guess+1)) > num then
        cur_guess
      else
      if cur_guess = past_guess then
        aux num (cur_guess-1) cur_guess 
      else
        aux num (max 1 ((cur_guess+past_guess)/2)) cur_guess 
  in aux x (x/2) 1  


let rec enumerate l = 
  let rec aux idx lis =
    match lis with
      [] -> []
    | h::t -> (idx,h)::aux (idx+1) t
  in aux 0 l


type rect =
  Square of int
 |Rectangle of int * int


let area r =
  match r with
    Square (i) -> i*i
  | Rectangle (u,v) -> u*v

let rotate r =
  match r with
    Square (_) -> r
  | Rectangle (u,v) -> Rectangle (v,u)

let width r =
  match r with 
    Square (i) -> i
  | Rectangle (i,j) -> i

let height r =
  match r with
    Square (j) -> j
  | Rectangle (i,j) -> j

let rotate_2_min_width r =
  if width r >= height r then rotate r else r

let rotate_2_max_width r =
  if width r <= height r then rotate r else r

let rec qs_r l =
    match l with       
      [] -> []
    | h::t -> qs_r (filter (fun i -> width i < width h ) t) @ h :: qs_r (filter (fun i -> width i>= width h) t)

let rotate_list rotate_fun l = qs_r (map rotate_fun l)









