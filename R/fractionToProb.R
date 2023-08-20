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