ThisBuild / version := "0.1.0-SNAPSHOT"

ThisBuild / scalaVersion := "3.1.3"

lazy val root = (project in file("."))
  .settings(
    name := "betting-calc-v2"
  )

libraryDependencies += "com.colofabrix.scala" %% "figlet4s-core" % "0.3.2"