# Design of breed feature

Only Kitties with different gender are allowed to breed a new kitten.
- Check if both involved kittens have different gender
- Implement gender method for kittens
    - If the DNA of a kitten is even, its gender is male, otherwise female

Rule for the generation of the child kitten:
- Create a DNA sequence based on the DNA of both parents
    - First, create an alternating sequence of the parent's DNAs (the even digits are reserved for the father's DNA, the odd ones for the mother's)
    - Second, create a random number between 0 and the length of the temporarily generated DNA. Let's call this number x.
    - Third, cut the last x positions of the DNA and move them to the beginning of the DNA strand.
