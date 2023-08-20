user.input <- function(prompt) {
  
  #' Reads the user input.
  #' 
  #' @param Character, The prompt message for the user. 
  #' 
  #' @description  A function to read the user input in both Interactive and 
  #' Non-Interactive mode. The function is similar to `readlines`. But due since 
  #' function `readline` would return `Na` when used non-interactively. To get the
  #' same functionality we would use `readLines` to read the input by the user.
  #' 
  #' @return Character, the user input would be returned. 
  #' 
  #' @example Name <- user.input('What is your Name? ')

  if (interactive()) {
    return(readline(prompt))
  } else {
    cat(prompt)
    return(readLines("stdin", n=1))
  }
}