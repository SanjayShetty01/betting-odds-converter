import scala.io.StdIn.*
import scala.util.{Failure, Success, Try}
object BettingOddsCalculator {

  private def isFraction(input: String): Boolean = {
    // Regular expression to match fractions (e.g., "1/2", "3/4")
    val fractionPattern = """^\d+/\d+$""".r
    fractionPattern.matches(input)
  }

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

    def decimalProbCalc(): Unit = {
    println(Console.BLUE +"Enter the Decimal Odds: ")

    val userEnteredDecimalValue : Try[Float] = Try(readFloat())

    userEnteredDecimalValue match{
      case Failure(value) =>
        println(Console.RED + "Please Enter a Numeric Value for Decimal Odds")


      case Success(value) =>
        println(Console.GREEN + "Calculating the Odds")
        val ans = ProbFunctions.decimalProb(value)
        println(Console.GREEN + s"The Implied Probability is $ans")
    }
  }

  def moneyProbCalc() : Unit = {
    println(Console.BLUE +"Enter the Moneyline: ")

    val userEnterMoneylinevalue : Try[Int] = Try(readInt())

    userEnterMoneylinevalue match {
      case Failure(value) =>
        println(Console.RED + "Please Enter a Integer Value for Moneyline Odds")

      case Success(value) =>
        println(Console.GREEN + "Calculating the Odds")
        val ans = ProbFunctions.moneylineProb(value)
        println(Console.GREEN + s"The Implied Probability is $ans")
    }
  }


  def fractionProbCalc() : Unit = {
    println(Console.BLUE + "Enter the Fractional Odds: ")

    val userEnterFractionValue: String = readLine()

    if(isFraction(userEnterFractionValue)){
      println(Console.GREEN + "Calculating the Odds")

        val decimalConvertedValue: Try[Float] = Try(fractionToDecimal(userEnterFractionValue))

        decimalConvertedValue match {
          case Failure(value) =>
            println(Console.RED +"Error: Invalid fraction or division by zero.")

          case Success(value) =>
            val ans = ProbFunctions.fractionProb(value)
            println(Console.GREEN + s"The Implied Probability is $ans")
        }
    } else{
      println(Console.RED +"Enter Fractional Odds in a Proper Format")
    }
  }

}
