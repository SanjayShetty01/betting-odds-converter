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
#' @include box


suppressMessages(if(!require(pacman)) install.packages("pacman"))
suppressMessages(pacman::p_load(Rfiglet, cli, box))

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
