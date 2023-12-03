import Data.Char (isDigit, ord)

main = do
  content <- readFile "../../../inputs/input02"
  putStrLn $ "The solution to star 1 is " ++ show (star1 content)
  putStrLn $ "The solution to star 2 is " ++ show (star2 content)

-- tag::star1[]
-- sum over ids of all lines which respect the limits
star1 :: String -> Int
star1 = sum . map fst . filter (allSmaller 12 13 14 . words . snd) . map splitId . lines
  where
    -- essentially a foldl which consumes two elements (count and color) at once
    allSmaller :: Int -> Int -> Int -> [String] -> Bool
    allSmaller _ _ _ [] = True
    allSmaller r g b (n : (c : _) : xs)
      | read n > v = False
      | otherwise = allSmaller r g b xs
      where
        v = case c of
          'r' -> r
          'g' -> g
          'b' -> b

-- end::star1[]

-- tag::star2[]
-- sum of product of maximum for every color for every line
star2 :: String -> Int
star2 = sum . map (prod' . maxByColor (0, 0, 0) . words . snd . splitId) . lines
  where
    prod' :: (Int, Int, Int) -> Int
    prod' (r, g, b) = r * g * b
    -- essentially a foldl which consumes two elements (count and color) at once
    maxByColor :: (Int, Int, Int) -> [String] -> (Int, Int, Int)
    maxByColor (r, g, b) [] = (r, g, b)
    maxByColor (r, g, b) (n' : (c : _) : xs)
      | c == 'r' = maxByColor (max r n, g, b) xs
      | c == 'g' = maxByColor (r, max g n, b) xs
      | c == 'b' = maxByColor (r, g, max b n) xs
      where
        n = read n'

-- end::star2[]

-- tag::helpers[]
-- extract Game id: ...draws... into (id, ...draws...)
splitId :: String -> (Int, String)
splitId line@(x : xs)
  | isDigit x = splitId' 0 line
  | otherwise = splitId xs
  where
    splitId' :: Int -> String -> (Int, String)
    splitId' id (x : xs)
      | isDigit x = splitId' (10 * id + (ord x - ord '0')) xs
    splitId' id (':' : ' ' : xs) = (id, xs)

-- end::helpers[]
