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
  def moneylineProb(moneyline: Float): Float = moneyline match {
    case moneylineValue if moneylineValue > 0 => roundToSecondDecimal((100 / (moneyline + 100.0f)) * 100)
    case _ => roundToSecondDecimal(((moneyline * -1) / ((moneyline * -1) + 100)) * 100)
  }

  /**
   * Calculate the implied probability from Decimal odds.
   *
   * @param decimal The Decimal odds as a Float.
   * @return The implied probability as a Float, rounded to two decimal places.
   */
  def decimalProb(decimal: Float): Float = roundToSecondDecimal((1.0f / decimal)* 100)

  /**
   * Calculate the implied probability from Fractional odds.
   *
   * @param fractions The Fractional odds as a Float.
   * @return The implied probability as a Float, rounded to two decimal places.
   */
  def fractionProb(fractions: Float): Float = roundToSecondDecimal((1.0f / fractions)* 100)

}
