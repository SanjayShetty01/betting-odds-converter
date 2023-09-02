import scala.io.StdIn._
import scala.util.{Failure, Success, Try}

/**
 * BettingOddsCalculator is a simple utility for calculating implied probabilities
 * from various types of odds: decimal, moneyline, and fractional.
 */
object BettingOddsCalculator {

  /**
   * Checks if a given string represents a valid fraction (e.g., "1/2", "3/4").
   *
   * @param input The input string to check.
   * @return True if the input is a valid fraction, false otherwise.
   */
  private def isFraction(input: String): Boolean = {
    // Regular expression to match fractions (e.g., "1/2", "3/4")
    val fractionPattern = """^\d+/\d+$""".r
    fractionPattern.matches(input)
  }

  /**
   * Converts a fractional odds string into a decimal odds value.
   *
   * @param fraction The fractional odds as a string (e.g., "1/2").
   * @return The equivalent decimal odds as a Float.
   * @throws IllegalArgumentException if division by zero is detected.
   */
  private def fractionToDecimal(fraction: String): Float = {
    val parts = fraction.split("/")
    val numerator = parts(0).toFloat
    val denominator = parts(1).toFloat

    if (denominator != 0) {
      numerator / denominator
    } else {
      throw new IllegalArgumentException("Division by zero")
    }
  }

  /**
   * Calculates the implied probability from decimal odds entered by the user.
   */
  def decimalProbCalc(): Unit = {
    println(Console.BLUE + "Enter the Decimal Odds: ")

    val userEnteredDecimalValue: Try[Float] = Try(readFloat())

    userEnteredDecimalValue match {
      case Failure(_) =>
        println(Console.RED + "Please Enter a Numeric Value for Decimal Odds")

      case Success(value) =>
        println(Console.GREEN + "Calculating the Odds")
        val ans = ProbFunctions.decimalProb(value)
        println(Console.GREEN + s"The Implied Probability is $ans")
    }
  }

  /**
   * Calculates the implied probability from moneyline odds entered by the user.
   */
  def moneyProbCalc(): Unit = {
    println(Console.BLUE + "Enter the Moneyline: ")

    val userEnterMoneylineValue: Try[Float] = Try(readFloat())

    userEnterMoneylineValue match {
      case Failure(_) =>
        println(Console.RED + "Please Enter an Integer Value for Moneyline Odds")

      case Success(value) =>
        println(Console.GREEN + "Calculating the Odds")
        val ans = ProbFunctions.moneylineProb(value)
        println(Console.GREEN + s"The Implied Probability is $ans")
    }
  }

  /**
   * Calculates the implied probability from fractional odds entered by the user.
   */
  def fractionProbCalc(): Unit = {
    println(Console.BLUE + "Enter the Fractional Odds: ")

    val userEnterFractionValue: String = readLine()

    if (isFraction(userEnterFractionValue)) {
      println(Console.GREEN + "Calculating the Odds")

      val decimalConvertedValue: Try[Float] = Try(fractionToDecimal(userEnterFractionValue))

      decimalConvertedValue match {
        case Failure(_) =>
          println(Console.RED + "Error: Invalid fraction or division by zero.")

        case Success(value) =>
          val ans = ProbFunctions.fractionProb(value)
          println(Console.GREEN + s"The Implied Probability is $ans")
      }
    } else {
      println(Console.RED + "Enter Fractional Odds in a Proper Format")
    }
  }
}
