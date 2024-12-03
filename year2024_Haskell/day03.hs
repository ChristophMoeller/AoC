import Text.ParserCombinators.ReadP
import Data.Char
import Data.Maybe

data Inst = Mul Int Int | Do | Dont deriving (Eq, Show)

pMul = (\x y -> Just $ Mul (read x) (read y)) <$> (string "mul(" *> munch1 isDigit <* string ",") <*> (munch1 isDigit <* string ")")
pDo = Just Do <$ string "do()"
pDont = Just Dont <$ string "don't()"
pInst = (pMul +++ pDo +++ pDont) <++ (Nothing <$ get)
parser = fmap catMaybes $ many pInst <* eof


partA :: [Inst] -> Int
partA (Mul a b:xs) = a*b + partA xs
partA (_:xs) = partA xs
partA _ = 0

partB :: Bool -> [Inst] -> Int
partB ignore (Mul a b:xs) = (if ignore then 0 else a*b) + partB ignore xs
partB _ (Do:xs) = partB False xs
partB _ (Dont:xs) = partB True xs
partB _ _ = 0

main :: IO()
main = do
  (instrs,_):_ <- readP_to_S parser <$> readFile "input/day03.txt";
  print (partA instrs)
  print (partB False instrs)
