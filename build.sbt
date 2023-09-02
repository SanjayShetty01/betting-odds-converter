ThisBuild / version := "0.1.0-SNAPSHOT"

ThisBuild / scalaVersion := "3.3.0"

lazy val root = (project in file("."))
  .settings(
    name := "betting-calc-v2"
  )

libraryDependencies ++= Seq(
  "com.colofabrix.scala" %% "figlet4s-core" % "0.3.0" cross CrossVersion.for3Use2_13,
  "com.colofabrix.scala" %% "figlet4s-effects" % "0.3.0" cross CrossVersion.for3Use2_13,
  "org.scalatest" %% "scalatest" % "3.2.15" % "test"
)

scalacOptions ++= Seq(
  "-deprecation",
  "-encoding", "UTF-8",
  "-feature",
  "-unchecked"
)