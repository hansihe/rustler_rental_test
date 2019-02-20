defmodule RentalTest.Nif do
  use Rustler, otp_app: [:rental_test], crate: "rentaltest_nif"

  def add(_a, _b), do: :erlang.nif_error(:nif_not_loaded)
end
