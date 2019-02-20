defmodule RentalTest.MixProject do
  use Mix.Project

  def project do
    [
      app: :rental_test,
      version: "0.1.0",
      elixir: "~> 1.7",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      compilers: [:rustler] ++ Mix.compilers,
      rustler_crates: rustler_crates()
    ]
  end

  def rustler_crates do
    [rentaltest_nif: [
      path: "native/rentaltest_nif",
      mode: (if Mix.env == :prod, do: :release, else: :debug),
    ]]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:rustler, "~> 0.19.0"}
      # {:dep_from_hexpm, "~> 0.3.0"},
      # {:dep_from_git, git: "https://github.com/elixir-lang/my_dep.git", tag: "0.1.0"},
    ]
  end
end
