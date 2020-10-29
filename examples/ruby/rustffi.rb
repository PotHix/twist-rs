require 'ffi'

module Twist
  extend FFI::Library
  ffi_lib 'target/debug/libtwist_rs.so'
  attach_function :search_ffi, [:string, :string], :string
end

puts Twist.search_ffi(ENV["auth"], "pothix")
