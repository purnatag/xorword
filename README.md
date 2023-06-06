# xorword
## The Input 
A signature from a p0f sensor, alphanumeric+special characters string

## The Algorithm
The code takes the following steps:
1. removes whitespace, commas from the signature to get string S0
2. converts the signature into binary (ascii to binary) as a Vec<u8>
3. breaks it into 32 bit chunks (last chunk might be smaller than 32 bit in length, is padded with zeros at the beginning)
4. XORs these chunks to get a final 32-bit string S1
  
5. Then, for i=4 to 2^14, for j = 1 to 1000,
  
    5-1. generate i length random alphanumeric string
    5-2. concatenate it with string S0, get string T0
    5-3. convert T0 to binary
    5-4. break into 32 bit chunks, pad the last chunk to 32 if necessary
    5-5. XOR these chunks to get 32-bit string T1
    5-6. calculate hamming distance between T1 and S1
    5-7. calculate average distance over 1000 iterations for each i
  
6. Then the program plots average distance versus i (chars added). 
  
## Observation
The average distance is seen to be converging to 6. The code writes the output to a text file and uses plotly to generate a line graph, that automatically loads an interactive graph to the browser. Clicking on the camera icon on the top right downloads a png snapshot of the plot.
