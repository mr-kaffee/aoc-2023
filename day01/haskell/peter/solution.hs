import Data.Char (isDigit, ord)
import Data.List (findIndex, isPrefixOf)

-- read input and output star1 and star2 solutions
main = do
  content <- readFile "../../../inputs/input01"
  putStrLn $ "The solution to star 1 is " ++ show (star score1 content)
  putStrLn $ "The solution to star 2 is " ++ show (star score2 content)

-- tag::star1[]
-- solution for both stars, use score1 or score2 as first argument
star :: (String -> Int) -> String -> Int
star f = sum . map f . lines

-- scoring function for star1
score1 :: String -> Int
score1 xs = 10 * score1' xs + score1' (reverse xs)
  where
    score1' :: String -> Int
    score1' (x : xs)
      | isDigit x = ord x - ord '0'
      | otherwise = score1' xs

-- end::star1[]

-- tag::star2[]
-- scoring function for star2
score2 :: String -> Int
score2 xs = 10 * score2' id xs + score2' reverse (reverse xs)
  where
    score2' :: (String -> String) -> String -> Int
    score2' f (x : xs)
      | isDigit x = ord x - ord '0'
      | otherwise = case findIndex ((`isPrefixOf` (x : xs)) . f) digits of
          Just digit -> digit
          Nothing -> score2' f xs
      where
        digits = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]

-- end::star2[]
