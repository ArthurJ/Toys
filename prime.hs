import Control.Monad
import Prelude

prime_list = [2,3,5]

is_prime :: Integer -> Integer -> [Integer] -> Bool
is_prime _ _ [] = True
is_prime x 0 lst = is_prime x (ceiling . sqrt . fromIntegral $ x) lst
is_prime x sx (y:ys)| mod x y == 0 = False
                    | (y > sx) = True
                    | otherwise = is_prime x sx ys

next_prime :: Integer -> [Integer] -> [Integer]
next_prime 0 lst = next_prime (last lst + 2 ) lst
next_prime x lst| is_prime x 0 lst = lst ++ [x]
                | otherwise = next_prime (x+2) lst

next_n_primes n lst | n <= 0 = lst
                    | otherwise = next_n_primes (n-1) (next_prime 0 lst) 

-- Mais conciso e direto, porém mais lento.
crivo :: [Integer] -> [Integer]
crivo (p:ps) = p : crivo [x | x <- ps, mod x p /= 0 ]

primos = crivo [2..]

main =  print $ next_n_primes 10000 prime_list
