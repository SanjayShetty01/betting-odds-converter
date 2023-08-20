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