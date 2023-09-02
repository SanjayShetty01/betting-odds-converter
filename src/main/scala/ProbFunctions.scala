import scala.math._
/**
 * Utility functions for calculating implied probabilities from different odds formats.
 */
object ProbFunctions {

  /**
   * Rounds a floating-point number to two decimal places.
   *
   * @param floatNum The input floating-point number.
   * @return The rounded floating-point number.
   */
  private def roundToSecondDecimal(floatNum: Float): Float = (floatNum * 100).round.toFloat / 100.0f

  /**
   * Calculate the implied probability from Moneyline odds.
   *
   * @param moneyline The Moneyline odds as a Float.
   * @return The implied probability as a Float, rounded to two decimal places.
   */
  def moneylineProb(moneyline: Float): Float = {
    var prob: Float = 0.0f

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

  /**
   * Calculate the implied probability from Decimal odds.
   *
   * @param decimal The Decimal odds as a Float.
   * @return The implied probability as a Float, rounded to two decimal places.
   */
  def decimalProb(decimal: Float): Float = {
    var prob: Float = 0.0f
    prob = 1.0f / decimal
    roundToSecondDecimal(prob)
  }

  /**
   * Calculate the implied probability from Fractional odds.
   *
   * @param fractions The Fractional odds as a Float.
   * @return The implied probability as a Float, rounded to two decimal places.
   */
  def fractionProb(fractions: Float): Float = {
    var prob: Float = 0.0f
    prob = 1.0f / fractions
    roundToSecondDecimal(prob)
  }
}
