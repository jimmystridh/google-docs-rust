#!/usr/bin/env ruby
# frozen_string_literal: true

script_dir = File.expand_path(__dir__)
exec(File.join(script_dir, 'sheets_manager'), *ARGV)
