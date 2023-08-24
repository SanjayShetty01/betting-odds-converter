import com.colofabrix.scala.figlet4s.unsafe._

object ProbCalculator {
  private def figletDrawer(str : String): Unit = {
    val builder = Figlet4s.builder()
    val figure = builder.render(str)
    figure.print()
  }


}

