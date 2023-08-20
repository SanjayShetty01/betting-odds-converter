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
