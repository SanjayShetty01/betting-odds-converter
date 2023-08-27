import scala.io.StdIn._
import scala.util.{Failure, Success, Try}
import com.colofabrix.scala.figlet4s.unsafe._

object MyApp extends App {
  private def figletDrawer(str: String): Unit = {
    val builder = Figlet4s.builder()
    val figure = builder.render(str)
    figure.print()
  }

  private def whichCalcDecider(choosen : Int): Unit = {
    choosen match {
      case 1 =>
        BettingOddsCalculator.moneyProbCalc()
      case 2 =>
        BettingOddsCalculator.decimalProbCalc()
      case 3 =>
        BettingOddsCalculator.fractionProbCalc()

      case _ =>
        println(Console.RED + "Please have your Choice between 1-3")
    }
  }

  println("")
  figletDrawer("Betting Odds Converter")
  println(Console.BLUE + "Calculates Implied Probability")

  private var exit : Boolean = false

  while(exit == false){
    val odds : List[String] = List("Moneyline to Implied probability",
                                    "Decimal Odds to Implied probability",
                                    "Fractional Odds to Implied probability"
    )

    println(Console.BLUE + "Select the Converter")

    for((odds, index) <- odds.zipWithIndex){
      val numbered : String = s"${index+1}."

      println(s"$numbered $odds")
    }

    val choice : Try[Int] = Try(readInt())

    choice match {
      case Success(value) =>
        whichCalcDecider(value)

      case Failure(value) =>
        println(Console.RED + "Please have your Choice between 1-3")
    }

    println("Press x to exit or any key to continue: ")
    val finalCall: String = readLine()

        exit = if (finalCall == "x" || finalCall == "X") then true else false
  }

}
