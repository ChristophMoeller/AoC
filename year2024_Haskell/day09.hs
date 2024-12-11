import System.IO
import Data.Char
import Data.Maybe
import qualified Data.Bifunctor
import Data.List

parseFile :: Int -> String -> [(Maybe Int, Int)]
parseFile id (x:y:zs) = (Just id, digitToInt x) : (Nothing, digitToInt y) : parseFile (id+1) zs
parseFile id [x] = [(Just id, digitToInt x)]
parseFile _ _ = []


frag2 :: [(Maybe Int, Int)] -> [(Maybe Int, Int)] -> [Int]
frag2 ((Just x, 0):zs) rs = frag2 zs rs
frag2 ((Just x, c):zs) rs = x : frag2 ((Just x, c-1):zs) rs
frag2 ((Nothing, 0):zs) rs = frag2 zs rs
frag2 ((Nothing, c):zs) ((Nothing, _):rs) = frag2 ((Nothing, c):zs) rs
frag2 ((Nothing, c):zs) ((Just _, 0):rs) = frag2 ((Nothing, c):zs) rs
frag2 ((Nothing, c):zs) ((Just x, d):rs) = x : frag2 ((Nothing, c-1):zs) ((Just x, d-1):rs)
frag2 _ _ = []

frag :: [(Maybe Int, Int)] -> [Int]
frag xs = take ( sum $ map onlyJusts xs ) $ frag2 xs (reverse xs)
  where
    onlyJusts (Just _, c) = c
    onlyJusts (Nothing, c) = 0

partA :: [(Maybe Int, Int)] -> Int
partA xs = sum $ zipWith (*) fragmented [0..]
  where
    fragmented = frag xs

fragBlock2 :: [(Maybe Int, Int)] -> (Int, Int) -> [(Maybe Int, Int)]
fragBlock2 ((Just x, cx):xs) y
  | (Just x, cx) == Data.Bifunctor.first Just y = (Just x, cx) : xs
  | otherwise = (Just x, cx) : fragBlock2 xs y
fragBlock2 ((Nothing, cx):xs) (y,cy)
  | cx < cy = (Nothing, cx) : fragBlock2 xs (y, cy)
  | otherwise = (Just y, cy) : (Nothing, cx - cy) : map (\(x,c) -> if x == Just y then (Nothing, c) else (x,c)) xs
fragBlock2 [] _ = []

fragBlock3 :: [(Maybe Int, Int)] -> [(Maybe Int, Int)] -> [(Maybe Int, Int)]
fragBlock3 xs [] = xs
fragBlock3 xs ((Nothing, _):ys) = fragBlock3 xs ys
fragBlock3 xs ((Just y, cy):ys) = fragBlock3 (fragBlock2 xs (y,cy)) ys

fragBlock :: [(Maybe Int, Int)] -> [Maybe Int]
fragBlock xs = concatMap (\(x, c) -> replicate c x ) (fragBlock3 xs (reverse xs))

partB :: [(Maybe Int, Int)] -> Int
partB xs = sum $ catMaybes $ zipWith (\x c -> (*) <$> x <*> Just c ) fragmented [0..]
  where
    fragmented = fragBlock xs


main :: IO()
main = do
  input <- parseFile 0 <$> readFile "input/day09.txt";
  print $ partA input
  print $ partB input
