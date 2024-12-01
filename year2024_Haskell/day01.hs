import System.IO
import Data.List
import Data.Maybe
import qualified Data.Map as Map
import Control.Applicative


parseLine :: String -> (Int, Int)
parseLine line = (read $ head w, read $ w !! 1) where w = words line

parseFile :: String -> ([Int], [Int])
parseFile = unzip . map parseLine . lines

partA :: [Int] -> [Int] -> Int
partA xs ys = sum $ absMap (sort xs) (sort ys)
  where
    absMap = zipWith ($) . map ((abs . ) . ( - ))

buildLookup :: [Int] -> Map.Map Int Int
buildLookup = foldr ( Map.alter updateOrInsert ) Map.empty
  where
    updateOrInsert = fmap (+1) . (<|> Just 0)

partB :: [Int] -> [Int] -> Int
partB xs ys = sum $ map (\x -> getFactor x * x ) xs
  where
    getFactor = fromJust . (<|> Just 0) . flip Map.lookup (buildLookup ys)

main :: IO ()
main = do {
  (xs, ys) <- parseFile <$> readFile "input/day01.txt";
  print (partA xs ys);
  print (partB xs ys);
}
