import Control.Monad
import Prelude

prime_list = [2,3,5,7,11,13,17,19,23,29,31,37,41,43,47,53,59,61,67,71,73,79,83,89,97,101]

is_prime :: Int -> Int -> [Int] -> Bool
is_prime _ _ [] = True
is_prime x 0 lst = is_prime x (ceiling . sqrt . fromIntegral $ x) lst
is_prime x sx (y:ys)| mod x y == 0 = False
                    | (y > sx) = True
                    | otherwise = is_prime x sx ys

next_prime :: Int -> [Int] -> [Int]
next_prime 0 lst = next_prime (last lst + 2 ) lst
next_prime x lst| is_prime x 0 lst = lst ++ [x]
                | otherwise = next_prime (x+2) lst

next_n_primes n lst | n <= 0 = lst
                    | otherwise = next_n_primes (n-1) (next_prime 0 lst) 

primos = filtraPrimos [2..]
    where filtraPrimos (p:xs) =
        p : filtraPrimos [x | x <- xs , 
            let s=(ceiling (sqrt (fromInteger x))),
            p>s || 
            x `mod` p /= 0
        ]

main =  print $ next_n_primes 5000 prime_list
