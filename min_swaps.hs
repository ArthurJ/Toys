{- What is the minimum amount of swaps between neighbors
    to group all equal elements on a list with 2 groups ({0,1})? -}

-- 0,0,0,3,4,2,2,3
testes = [[0, 0, 0, 0, 0, 0],
          [1, 1, 1, 0, 0, 0],
          [0, 0, 0, 1, 1, 1],
          [1, 0, 1, 0, 1, 0],
          [0, 1, 1, 0, 1, 0],
          [0, 0, 1, 0, 0, 0],
          [0, 0, 0, 1, 0, 0],
          [0, 0, 1, 1, 1, 0]]


push :: Ord a => [a] -> [a] -> (a -> a -> Bool) -> Int -> (Int, [a])
push ref []  _ count = (count, reverse ref)
push ref [a] _ count = (count, reverse (a:ref))
push ref (x:y:xs) comp count
    | comp x y = push ref (y:x:xs) comp (count+1)
    | otherwise = push (x:ref) (y:xs) comp count


-- count and past_count MUST start with different values
sprintInternal :: Ord a => (a -> a -> Bool) -> [a] -> Int -> Int -> (Int, [a])
sprintInternal comp lst count past_count
    | count == past_count = (count, nlist)
    | count == 0 = sprintInternal comp nlist ncount count
    | otherwise = sprintInternal comp nlist (ncount+count) count
    where (ncount, nlist) = push [] lst comp 0


sprint :: Ord a => (a -> a -> Bool) -> [a] -> (Int, [a])
sprint comp lst = sprintInternal comp lst 0 (-1)


main = do
    let menor = map (sprint (<)) testes
    let maior = map (sprint (>)) testes
    print $ zipWith min menor maior
