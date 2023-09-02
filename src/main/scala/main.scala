import scala.io.StdIn._
import scala.util.{Failure, Success, Try}
import com.colofabrix.scala.figlet4s.unsafe._

/**
 * Betting Odds Converter is a command-line utility for calculating implied probabilities from different odds formats.
 */
object MyApp extends App {

  /**
   * Draws a Figlet-style text art of a string.
   *
   * @param str The string to render as text art.
   */
  private def figletDrawer(str: String): Unit = {
    val builder = Figlet4s.builder()
    val figure = builder.render(str)
    figure.print()
  }

  /**
   * Decides which odds calculation method to invoke based on the user's choice.
   *
   * @param chosen The user's choice as an integer.
   */
  private def whichCalcDecider(chosen: Int): Unit = {
    chosen match {
      case 1 =>
        BettingOddsCalculator.moneyProbCalc()
      case 2 =>
        BettingOddsCalculator.decimalProbCalc()
      case 3 =>
        BettingOddsCalculator.fractionProbCalc()
      case _ =>
        println(Console.RED + "Please choose a valid option between 1-3.")
    }
  }

  println("")
  figletDrawer("Betting Odds Converter")
  println(Console.BLUE + "Calculates Implied Probability")

  private var exit: Boolean = false

  while (!exit) {
    val odds: List[String] = List(
      "Moneyline to Implied probability",
      "Decimal Odds to Implied probability",
      "Fractional Odds to Implied probability"
    )

    println(Console.BLUE + "Select the Converter")

    for ((odds, index) <- odds.zipWithIndex) {
      val numbered: String = s"${index + 1}."

      println(s"$numbered $odds")
    }

    val choice: Try[Int] = Try(readInt())

    choice match {
      case Success(value) =>
        whichCalcDecider(value)

      case Failure(value) =>
        println(Console.RED + "Please choose a valid option between 1-3.")
    }

    println("Press 'x' to exit or any other key to continue: ")
    val finalCall: String = readLine()

    exit = finalCall.equalsIgnoreCase("x")
  }

}
