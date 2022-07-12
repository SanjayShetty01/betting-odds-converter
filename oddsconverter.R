#!/usr/bin/Rscript 

#' Betting Odds Converter.
#' 
#' @description A CLI based Application which allows to convert various type 
#' of Betting odds to Implied Probability. The implied probability indicates you 
#' the probability of winning assigned by the betting market.
#' 
#' @author Sanjaya J Shetty
#' 
#' @include pacman 
#' @include  Rfiglet 
#' @include  cli


suppressMessages(if(!require(pacman)) install.packages('pacman'))
suppressMessages(pacman::p_load(Rfiglet, cli))


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

moneylineToProb <- function(){
  
  #' Converts  Moneyline to Implied Probabilities.
  #' 
  #' The function allows you to covert the American Moneyline to implied
  #' probabilities.
  #' 
  #' @details 
  #' American Moneyline would be denoted by a integer number. The American
  #' Moneyline, if positive number states the amount you would win if you wager $100,
  #' But if it's negative, then it would state the amount you would require wager to win
  #' $100. 
  #' 
  #' @return numeric, The implied probability is returned.
  
  suppressWarnings(moneyline <- as.integer(user.input(prompt = 'Enter the Moneyline: ')))
  
  if(is.na(moneyline)){
    cli_alert_danger('Please Enter a Numeric Value for Moneyline')
    
  }else{
  
      if(moneyline > 0){
        prob <- moneyline/(moneyline+100)
        return(round(prob*100,2))
}     else{
        prob <- (moneyline*-1)/((moneyline*-1)+100)
        return(round(prob*100,2))
  }
}
}


decimalToProb <- function(){
  
  #' Converts decimal odds to Implied Probability
  #' 
  #' The function allows you convert decimal odds entered by the user into implied
  #' probability. 
  #' 
  #' @details 
  #' The decimal odds quote the potential returns that would be paid if the bet
  #' succeeds in your favour. 
  #' 
  #' @return numeric, The implied probability is returned. 
  
  suppressWarnings(decimal <- as.integer(user.input(prompt = 'Enter the Decimal Odds: ')))
  
  if(is.na(decimal)){
    cli_alert_danger('Please Enter a Numeric Value for Decimal Odds')
    
  }else{
  prob <- 1/decimal
  return(round(prob*100,2))
}
}


fractionToProb <- function(fraction){
  
  #' Converts fraction odds to Implied Probability.
  #'
  #' @details 
  #' Like the decimal odds, the fractional bet also quote the potential returns that 
  #' would be paid if the bet succeeds in your favour. Albeit in a fractional format. 
  #'
  #' @return numeric, The implied probability is returned. 
 
  suppressWarnings(fraction <- as.integer(user.input(prompt = 'Enter the Fractional Odds: ')))

  if(is.na(fraction)) {
    cli_alert_danger('Please Enter a Integer for Fractional Odds')
} else{  
    prob <- 1/(fraction+1)
    return(round(prob*100,2))
}
}

cli_h1('')    
figlet("Betting Odds Converter")
cli_text('Calculates Implied Probability')
exitOut = FALSE
while(exitOut == FALSE){
  
  cli_h1('Select the Converter')
  
  fun <- function() {
    cli_ol()
    cli_li("Moneyline to Implied probability")
    cli_li("Decimal Odds to Implied probability")
    cli_li("Fractional Odds to Implied probability")
    cli_end()
  }
  fun()
  
  cli_h1('')              
  suppressWarnings(choice <- (user.input(prompt = 'Enter your Choice: ')))
  
  if(choice == '1' | choice == '2' | choice == '3'){
    ans = switch(choice,
                 '1' = sprintf('The Implied Probability is %.02f', moneylineToProb()),
                 '2' = sprintf('The Implied Probability is %.02f',decimalToProb()),
                 '3' = sprintf('The Implied Probability is %.02f',fractionToProb()))
    
    
    cli_text(ans)  
  } else{
    cli_alert_warning('Please have your Choice between 1-3')
    
  }
  
  finalCall <- user.input(prompt = 'Press x to exit or any key to continue: ')
  
  exitOut = ifelse(finalCall == 'x', TRUE, FALSE)
  
  
}