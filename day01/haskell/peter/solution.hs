import Data.Char (isDigit, ord)

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
score2 xs = 10 * score2' xs + score2'' (reverse xs)
  where
    score2' :: String -> Int
    score2' (x : xs)
      | isDigit x = ord x - ord '0'
    score2' ('z' : 'e' : 'r' : 'o' : xs) = 0
    score2' ('o' : 'n' : 'e' : xs) = 1
    score2' ('t' : 'w' : 'o' : xs) = 2
    score2' ('t' : 'h' : 'r' : 'e' : 'e' : xs) = 3
    score2' ('f' : 'o' : 'u' : 'r' : xs) = 4
    score2' ('f' : 'i' : 'v' : 'e' : xs) = 5
    score2' ('s' : 'i' : 'x' : xs) = 6
    score2' ('s' : 'e' : 'v' : 'e' : 'n' : xs) = 7
    score2' ('e' : 'i' : 'g' : 'h' : 't' : xs) = 8
    score2' ('n' : 'i' : 'n' : 'e' : xs) = 9
    score2' (x : xs) = score2' xs
    score2'' :: String -> Int
    score2'' (x : xs)
      | isDigit x = ord x - ord '0'
    score2'' ('o' : 'r' : 'e' : 'z' : xs) = 0
    score2'' ('e' : 'n' : 'o' : xs) = 1
    score2'' ('o' : 'w' : 't' : xs) = 2
    score2'' ('e' : 'e' : 'r' : 'h' : 't' : xs) = 3
    score2'' ('r' : 'u' : 'o' : 'f' : xs) = 4
    score2'' ('e' : 'v' : 'i' : 'f' : xs) = 5
    score2'' ('x' : 'i' : 's' : xs) = 6
    score2'' ('n' : 'e' : 'v' : 'e' : 's' : xs) = 7
    score2'' ('t' : 'h' : 'g' : 'i' : 'e' : xs) = 8
    score2'' ('e' : 'n' : 'i' : 'n' : xs) = 9
    score2'' (x : xs) = score2'' xs
-- end::star2[]
