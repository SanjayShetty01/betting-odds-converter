import scala.math._
object ProbFunctions {

  private def roundToSecondDecimal(floatNum : Float) : Float = (floatNum * 100).round.toFloat / 100.0f

  def moneylineProb(moneyline : Float) : Float = {
    var prob : Float = 0.0f

    if (moneyline > 0) {
      prob = (moneyline / (moneyline + 100.0f)) * 100
      prob = roundToSecondDecimal(prob)
    }
    else {
      prob = ((moneyline * -1) / ((moneyline * -1) + 100)) * 100
      prob = roundToSecondDecimal(prob)
    }

    prob
  }

  def decimalProb(decimal : Float) : Float = {
    var prob : Float = 0.0f
    prob = 1.0f / decimal
    roundToSecondDecimal(prob)
  }

  def fractionProb(fractions: Float) : Float = {
    var prob : Float = 0.0f
    prob = 1.0f / fractions
    roundToSecondDecimal(prob)
  }
}



