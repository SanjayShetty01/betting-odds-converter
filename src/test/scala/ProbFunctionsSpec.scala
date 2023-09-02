import org.scalatest.flatspec.AnyFlatSpec
import org.scalatest.matchers.should.Matchers

class ProbFunctionsSpec extends AnyFlatSpec with Matchers {

  "moneylineProb" should "calculate implied probability for positive moneyline" in {
    val result = ProbFunctions.moneylineProb(200.0f)
    result shouldEqual 33.33f
  }

  it should "calculate implied probability for negative moneyline" in {
    val result = ProbFunctions.moneylineProb(-150.0f)
    result shouldEqual 60.0f
  }

  "decimalProb" should "calculate implied probability from decimal odds" in {
    val result = ProbFunctions.decimalProb(1.5f)
    result shouldEqual 66.67f
  }

  "fractionProb" should "calculate implied probability from fractional odds" in {
    val result = ProbFunctions.fractionProb(2.0f)
    result shouldEqual 50.0f
  }
}

