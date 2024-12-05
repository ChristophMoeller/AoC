import System.IO
import Data.List
import Data.Maybe
import qualified Data.Set as Set
import Text.ParserCombinators.ReadP
import Data.Char
import Control.Applicative
import Data.Ord


pOrd = (\x y -> (read x, read y ) ) <$> munch1 isDigit <* string "|" <*> munch1 isDigit
pUps = sepBy (read <$> munch1 isDigit) (string ",")

parser = readP_to_S $ (,) <$> sepBy pOrd (satisfy isSpace) <* munch1 isSpace <*> sepBy pUps (satisfy isSpace) <* eof

parseFile :: String -> (Set.Set (Int,Int), [[Int]])
parseFile input = (Set.fromList ords, ups)
  where
    ((ords, ups),_):_ = parser input;

-- isOrdered :: Set.Set (Int,Int) -> [Int] -> Bool
-- isOrdered ords (x:xs) = not (any (\y -> Set.member (y,x) ords) xs) && isOrdered ords xs
-- isOrdered ords _ = True

isOrdered :: Set.Set (Int,Int) -> [Int] -> Maybe (Int, Int)
isOrdered ords (x:xs) = fmap (\i -> (x,i+1)) (findIndex (\y -> Set.member (y,x) ords) xs) <|> fmap (\(x,y) -> (x+1,y+1)) (isOrdered ords xs)
isOrdered ords _ = Nothing

data Page = Page (Set.Set (Int,Int)) Int

instance Eq Page where
  Page _ x == Page _ y = x == y

instance Ord Page where
  (<=) :: Page -> Page -> Bool
  (Page o x) <= (Page _ y) = Set.member (x,y) o || x == y


partA :: Set.Set (Int,Int) -> [[Int]] -> Int
partA ords = sum . map (\up -> up !! div (length up) 2 ) . filter (isNothing . isOrdered ords)

partB :: Set.Set (Int,Int) -> [[Int]] -> Int
partB ords = sum . map ( (\(Page _ x) -> x) . (\up -> up !! div (length up) 2 ) . sort . map (Page ords) ) . filter (isJust . isOrdered ords)

main :: IO()
main = do
  (ords, ups) <- parseFile <$> readFile "input/day05.txt";
  print $ partA ords ups
  print $ partB ords ups

