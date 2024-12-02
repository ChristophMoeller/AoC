import Data.Ord

parseFile :: String -> [[Int]]
parseFile = map (map read . words) . lines


isSafeOrdered :: Ordering -> Bool -> [Int] -> Bool
isSafeOrdered cmp rem (x:y:z:xs) = compare x y == cmp && abs (x - y) <= 3 && isSafeOrdered cmp rem (y:z:xs) || ( not rem && compare x z == cmp && abs (x - z) <= 3 && isSafeOrdered cmp True (z:xs) )
isSafeOrdered cmp rem [x,y] = not rem || ( compare x y == cmp && abs (x - y) <= 3 )
isSafeOrdered cmp rem _ = True

isSafe :: Bool -> [Int] -> Bool
isSafe rem (x:xs) = isSafeOrdered LT rem (x:xs) || isSafeOrdered GT rem (x:xs) || (not rem && isSafe True xs)
isSafe _ _ = True

main :: IO()
main = do 
  xss <- parseFile <$> readFile "input/day02.txt";
  print ((length . filter (isSafe True)) xss)
  print ((length . filter (isSafe False)) xss)
