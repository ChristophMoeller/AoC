import Data.List
import Data.Maybe



checkMaskOrigin :: [[Maybe Char]] -> [[Char]] -> Bool
checkMaskOrigin (m:ms) (x:xs) = length m <= length x && all (\(a,b) -> a == Just b || isNothing a) (zip m x) && checkMaskOrigin ms xs
checkMaskOrigin [] _ = True
checkMaskOrigin m [] = False

checkMaskFirstRow :: [[Maybe Char]] -> [[Char]] -> Int
checkMaskFirstRow m xs = ( if checkMaskOrigin m xs then 1 else 0 ) +
  ( if not (null (head xs)) then checkMaskFirstRow m $ map tail xs else 0 )

checkMask :: [[Maybe Char]] -> [[Char]] -> Int
checkMask m (x:xs) = checkMaskFirstRow m (x:xs) + checkMask m xs
checkMask _ _ = 0


mask1 = [map Just "XMAS"]
mask2 = [map Just "SAMX"]
mask3 = transpose mask1
mask4 = transpose mask2

dmask1 = [ [if x == y then Just x else Nothing | x <- "XMAS"] | y <- "XMAS"]
dmask2 = [ [if x == y then Just x else Nothing | x <- "SAMX"] | y <- "XMAS"]
dmask3 = [ [if x == y then Just x else Nothing | x <- "XMAS"] | y <- "SAMX"]
dmask4 = [ [if x == y then Just x else Nothing | x <- "SAMX"] | y <- "SAMX"]

amasks = [mask1, mask2, mask3, mask4, dmask1, dmask2, dmask3, dmask4]

bmask1 = [[Just 'M', Nothing, Just 'S'], [Nothing, Just 'A', Nothing], [Just 'M', Nothing, Just 'S']]
bmask2 = [[Just 'S', Nothing, Just 'M'], [Nothing, Just 'A', Nothing], [Just 'S', Nothing, Just 'M']]
bmask3 = transpose bmask1
bmask4 = transpose bmask2

bmasks = [bmask1, bmask2, bmask3, bmask4]

main :: IO()
main = do
  input <- lines <$> readFile "input/day04.txt";
  print $ sum $ map checkMask amasks <*> [input]
  print $ sum $ map checkMask bmasks <*> [input]
