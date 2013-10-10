#!/usr/bin/env ruby

require 'fileutils'
require 'optparse'
require 'date'
require 'parsedate'
require 'time'

# parse options
$options = {}
optparse = OptionParser.new do |opts|
  opts.banner = "jobber - job time tracker\nUsage: jobber [options]\n"

  opts.separator ""
  opts.separator "Job creation:"
  
  opts.on( '-s', '--start [TIME]', 'Start work now (or at given TIME)' ) do |v| 
    $options[:start] = true
    $options[:start_time] = v
  end
  opts.on( '-e', '--end [TIME]', 'End work (at TIME)' ) do |v| 
    $options[:end] = true
    $options[:end_time] = v 
  end
  opts.on( '-d', '--duration HOURS', 'Work time in hours' ) do |v| 
    $options[:duration] = v
  end
  opts.on( '-m', '--message [MESSAGE]', 'Add message to job' ) do |v| 
    $options[:message] = true
    $options[:message_text] = v
  end

  opts.separator ""
  opts.separator "    TIME can be one of the following values:"
  opts.separator ""
  opts.separator "    now              now"
  opts.separator "    4:10-            4 hours and 10 minutes ago"
  opts.separator "    1h+              in 1 hour"
  opts.separator "    14:10            today at 14:10"
  opts.separator "    8/1/,14:10       at 1st of August this year at 14:10"
  opts.separator "    mon,14:10        last monday at 14:10"
  opts.separator "    yesterday,14:10  yesterday at 14:10"

  opts.separator ""
  opts.separator "Job editing:"

  opts.on( '-D', '--drop POS', 'Drop job at given position' ) do |v| 
    $options[:drop] = v
  end
  opts.on( '-j', '--join POS1,POS2[,...]', Array, 'Join two or more jobs at the given positions' ) do |v| 
    if v.size < 2
      puts opts
      exit
    end
    $options[:join] = v
  end

  opts.separator ""
  opts.separator "Reporting:"

  opts.on( '-l', '--list [TIME|COUNT]', 'List existing jobs' ) do |v| 
    $options[:list] = true
    $options[:list_filter] = v
  end
  opts.on( '-t', '--total [TIME|COUNT]', 'Measure exisiting jobs' ) do |v| 
    $options[:total] = true
    $options[:list_filter] = v
  end
  $options[:report] = false 
  opts.on( '-r', '--report', 'Report existing jobs' ) do  
    $options[:report] = true
  end
  $options[:resolution] = 0.25
  opts.on( '-R', '--resolution RESOLUTION', 'Time resolution (default: 0.25)' ) do |v| 
    $options[:resolution] = v
  end
  opts.on( '-M', '--money RATE', 'Display hours*RATE' ) do |v| 
    $options[:rate] = v
  end
  
  opts.separator ""
  opts.separator "Miscellaneous:"
  
  $options[:filename] = "jobber.dat"
  opts.on( '-f', '--file FILENAME', 'file to use (default: jobber.dat)' ) do |v| 
    $options[:filename] = v
  end  
  $options[:verbose] = false
  opts.on( '-v', '--verbose', 'Output more information' ) do
    $options[:verbose] = true
  end
  opts.on( '-h', '--help', 'Display this screen' ) do
    puts opts
    exit
  end

  if ARGV.size < 1 
    puts opts
    exit
  end
end
optparse.parse!

# provide some ANSI escape sequences into String to colorize output
class String
  def black;          "\033[30m#{self}\033[0m" end
  def red;            "\033[31m#{self}\033[0m" end
  def green;          "\033[32m#{self}\033[0m" end
  def brown;          "\033[33m#{self}\033[0m" end
  def blue;           "\033[34m#{self}\033[0m" end
  def magenta;        "\033[35m#{self}\033[0m" end
  def cyan;           "\033[36m#{self}\033[0m" end
  def gray;           "\033[37m#{self}\033[0m" end
  def bg_black;       "\033[40m#{self}\0330m"  end
  def bg_red;         "\033[41m#{self}\033[0m" end
  def bg_green;       "\033[42m#{self}\033[0m" end
  def bg_brown;       "\033[43m#{self}\033[0m" end
  def bg_blue;        "\033[44m#{self}\033[0m" end
  def bg_magenta;     "\033[45m#{self}\033[0m" end
  def bg_cyan;        "\033[46m#{self}\033[0m" end
  def bg_gray;        "\033[47m#{self}\033[0m" end
  def bold;           "\033[1m#{self}\033[22m" end
  def reverse_color;  "\033[7m#{self}\033[27m" end
end

# add a "humanized" output format to DateTime
class DateTime
  def to_h
    return self.strftime("%a %b %d %Y, %H:%M") 
  end
end

class Range
  def intersection(other)
    return nil if (self.max < other.begin or other.max < self.begin) 
    [self.begin, other.begin].max..[self.max, other.max].min
  end
  alias_method :&, :intersection
end

# a single job 
class Job
  attr_reader :start, :end
  attr_accessor :message
  # from scratch
  def initialize s=0, e=0, d=""
    @start = s
    @end = e 
    @message = d
  end
  # compare start times
  def <=> other
    return @start <=> other.start
  end
  # intersect with timespan of another job
  def intersect other 
    return (@start..@end) & (other.start..other.end)
  end
  # create from quoted values in CSV line
  def self.from_s line
    a = line.split(';')
    a.each do |v| 
      v.chomp!('"')
      v.reverse!
      v.chomp!('"')
      v.reverse!
    end
    return Job.new(DateTime.parse(a[0]),(a[1] == '0')?0:DateTime.parse(a[1]),a[2].gsub(/\\n/,"\n"))
  end
  # pack CSV line
  def pack
    return ["\"#{@start}\"", "\"#{@end}\"", @message.nil? ? "\"\"" : "\"#{@message.gsub(/\n/,'\\n')}\""].join(";")
  end
  # humanized version of data
  def to_s
    s = ""
    s << "  Start: #{@start.to_h.green}\n"
    if finished?
      s << "    End: #{@end.to_h.red}\n" 
      s << "  Hours: #{hours}\n" 
      s << "  Costs: #{hours*$options[:rate]}\n" if $options[:rate]
    end
    first = true
    if !@message.nil? and !message.empty?
      @message.each_line do |l|
        s << "Message: #{l.bold}" if first
        s << "         #{l.bold}" if !first
        first = false if first
      end
      s << "\n"
    end
    s << "\n"
    return s
  end
  # check if job timspan is valid
  def self.check(s,e)
    return s < e 
  end
  # set start time
  def start=(s)
    @start = s if Job.check(s,@end)
  end
  # set end time
  def end=(e)
    @end = e if Job.check(@start,e)
  end
  # get year of start time
  def year
    return @start.year
  end
  # get month of start time
  def month
    return @start.month
  end
  # get day of month from start time
  def mday
    return @start.mday
  end
  # get hours without rounding to resolution
  def hours_exact
    e = DateTime.now
    e = @end if @end != 0
    return (e - @start) * 24
  end
  # get worked hours rounded to resolution (if job is running end time is now)
  def hours
    return (hours_exact/$options[:resolution].to_f).round*$options[:resolution].to_f
  end
  # check if start time has been set
  def valid?
    return @start != 0
  end
  # check if job is finished
  def finished?
    return @end != 0
  end
end

# enhance Regexp
class Regexp
  # check if t completely matches
  def check t
    return t == match(t).to_s
  end
end

# parses a string to a DateTime
def parsetime t, allow_date_only=false
  # accepted formats
  reg_reltime1 = /\d{1,2}:\d{1,2}(\+|-)/
  reg_reltime2 = /\d{1,2}(h|m)(\+|-)/
  reg_reltime = /#{reg_reltime1}|#{reg_reltime2}/
  reg_abstime = /\d{1,2}:\d{1,2}/
  reg_time = /#{reg_abstime}|#{reg_reltime}/
  reg_dategerman = /\d{1,2}\.\d{1,2}((\.\d{1,4})|\.)?/
  reg_dateenglish = /\d{1,2}\/\d{1,2}(\/\d{1,4})/
  reg_weekday = /mon|tue|wed|thu|fri|sat|sun|yesterday/
  reg_date = /#{reg_dategerman}|#{reg_dateenglish}|#{reg_weekday}/
  reg_datetime = /#{reg_date},#{reg_abstime}/
  reg_timedate = /#{reg_abstime},#{reg_date}/
  reg_now = /now/
  reg_dateandtime = /#{reg_datetime}|#{reg_timedate}|#{reg_now}/

  print "parse time '#{t}': " if $options[:verbose]
  if reg_now.check(t)
    puts "now #{t}" if $options[:verbose]
    return DateTime.now
  elsif reg_reltime1.check(t)
    puts "relative time 1 #{t}" if $options[:verbose]
    a = t.split(':')
    a[1].chomp!("+");
    a[1].chomp!("-");
    return DateTime.now - a[0].to_f/24 - a[1].to_f/24/60 if t.end_with?('-')
    return DateTime.now + a[0].to_f/24 + a[1].to_f/24/60 if t.end_with?('+')
  elsif reg_reltime2.check(t)
    puts "relative time 2 #{t}" if $options[:verbose]
    f = -1 if t.end_with?('-')
    f = 1 if t.end_with?('+')
    t.chomp!("-")
    t.chomp!("+")
    return DateTime.now + f*t.chomp("h").to_f/24 if t.end_with?('h')
    return DateTime.now + f*t.chomp("m").to_f/24/60 if t.end_with?('m')
  elsif reg_abstime.check(t)
    puts "absolute time #{t}" if $options[:verbose]
    a = t.split(':')
    tim = DateTime.now
    tim -= tim.hour.to_f/24 + tim.min.to_f/24/60
    tim += a[0].to_f/24 + a[1].to_f/24/60 
    tim -= 1 if (tim - DateTime.now) > 0.5 
    return tim
  elsif reg_dateandtime.check(t) or (allow_date_only and reg_date.check(t))
    print "date and time:" if $options[:verbose]
    a = t.split(',')
    rt = Time.at(0)
    rd = DateTime.new
    a.each do |v| 
      if reg_dategerman.check(v)
        print " german date" if $options[:verbose]
        b = v.split('.')
        b[2] = DateTime.now.year if b.size < 2 or b[2].nil?
        rd = DateTime.new(b[2].to_i,b[1].to_i,b[0].to_i)
      end
      if reg_dateenglish.check(v)
        print " english date" if $options[:verbose]
        b = v.split('/')
        b[2] = DateTime.now.year if b.size < 2 or b[2].nil?
        rd = DateTime.new(b[2].to_i,b[0].to_i,b[1].to_i)
      end
      if reg_weekday.check(v)
        print " weekday" if $options[:verbose]
        rd = DateTime.local(Time.now.year,Time.now.month,Time.now.mday)
        if v == "yesterday"
          rd -= 1
        else
          w = [ "sun", "mon", "tue", "wed", "thu", "fri", "sat" ]
          while w[rd.wday] != v
            rd -= 1
          end
        end
      end
      if reg_abstime.check(v)
        print " abstime" if $options[:verbose]
        a = v.split(':')
        rt = Time.at(60*60*a[0].to_i + 60*a[1].to_i)
      end
      puts if $options[:verbose]
    end
    return DateTime.new(rd.year,rd.month,rd.mday,rt.utc.hour,rt.utc.min)
  end
  puts "invalid" if $options[:verbose]
  return nil
end

# small multi line editor
def multi_gets all_text=""
  while (text = gets) != "\n"
    all_text << text
  end
  all_text.strip
end

# enter a message or take it from program options
def enter_message force=false, msg="Please enter a message (empty line quits):"
  if force or $options[:message]
    if $options[:message_text].nil?
      puts msg
      return multi_gets.strip
    else
      return $options[:message_text].gsub(/\\n/,"\n")
    end
  end
end

# format hours
def fmthours h
  return h.round.to_s + ":" + ((h - h.round)*60).round.to_s.rjust(2,'0')
end

# ends a running job
def endjob e, msg="Ending job:"
  if $jobs.empty? or $jobs.last.end != 0
    puts "There is no open job!".red
    return false 
  elsif Job.check($jobs.last.start,e)
    if $jobs.last.message.nil? or $jobs.last.message.empty?
      $jobs.last.message = enter_message true
    end
    puts msg.brown
    $jobs.last.end = e
    puts "    Pos: #{$jobs.size}"
    puts $jobs.last
    return true
  else
    print "End time is ahead of start time! Please retry:"
    return false  
  end
end

# start a job (asks user to stop a running job)
def startjob s, msg="Starting new job:"
  if !$jobs.last.nil? and $jobs.last.end == 0
    puts "There is still an open job!".red
    puts $jobs.last
    print "Do you want to close this job first (enter time or nothing to cancel)? "
    while 
      answer = gets.strip

      if answer.empty?
        puts "Canceling job start."
        puts "Running job remains open!"
        exit
      else 
        t = parsetime(answer)
        if t.nil?
          print "Please enter a valid time:"
        else
          break if endjob t 
        end
      end
    end
  end
  job = Job.new(s,0,"")
  job.message = enter_message
  if !msg.nil?
    puts msg.brown 
    puts "    Pos: #{$jobs.size+1}"
    puts job
    puts "Stop it with -e when you're finished" if $options[:verbose]
  end
  $jobs << job
end

# remove job
def drop pos
  puts $jobs[pos-1]
  print "Do you really want to delete this job (y/N)?"
  if gets.strip.casecmp("y") == 0
    puts "Deleting job ##{pos}".brown
    $jobs.delete_at(pos-1)
  else
    puts "Deletion canceled."
    exit
  end
end

# join two jobs by merging their attributes
def join a
  a.sort!
  job = $jobs[a.first]
  puts "Join #{a.size} jobs:"
  job.message = a.collect{ |i| $jobs[i-1].message }.join("\n")
  hours = 0
  a.each do |i|
    j = $jobs[i-1]
    hours += j.hours
    puts j
    job.start = (job.start < j.start) ? job.start : j.start
    job.end = (job.end > j.end) ? job.end : j.end
  end
  puts "Into this job:"
  puts job
  puts "You will add #{job.hours-hours} hours!" if job.hours > hours
  puts "You will lose #{job.hours-hours} hours!" if job.hours < hours
  print "Do you really want to merge #{a.size} jobs into the above job (y/N)?"
  if gets.strip.casecmp("y") == 0
    puts "Merge jobs #{a.join(',')}...".brown
    $jobs[a.first] = job
    a.drop(1).reverse.each { $jobs.delete_at(a[i]) }
  else
    puts "Join canceled"  
  end
end

# list jobs within list_filter from options
def listjobs totals_only=false
  t = parsetime($options[:list_filter],true)
  n = $options[:list_filter].to_i if t.nil? and !$options[:list_filter].nil?
  puts "Listing jobs since #{t}:" if !t.nil? and $options[:verbose]
  pos = 0
  count = 0
  hours = 0
  $jobs.each do |j| 
    pos += 1
    next if !t.nil? and j.start < t
    next if !n.nil? and pos <= $jobs.size-n
    if !totals_only 
      puts "    Pos: #{pos}"
      puts j
    end
    count += 1
    hours += j.hours
  end
  txt = "Total: #{count} job(s), #{hours.to_s.bold} hrs."
  txt << " / $#{(hours*$options[:rate].to_f).to_s.bold}" if !$options[:rate].nil?
  puts txt
  puts "Job running since #{fmthours($jobs.last.hours_exact).green} hour(s)!" if !$jobs.empty? and !$jobs.last.finished?
end

# report monthly
def report
  puts
  a = []
  $jobs.each do |j|
    a[j.year] = [] if a[j.year].nil?
    a[j.year][j.month] = [] if a[j.year][j.month].nil?
    a[j.year][j.month][j.mday] = 0 if a[j.year][j.month][j.mday].nil?
    a[j.year][j.month][j.mday] += j.hours
  end
  weekdays = ["sun", "mon", "tue", "wed", "thu", "fri", "sat", "week"]
  months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec" ]
  col_width = 8
  line_width = col_width*8
  all_hours = 0
  week_hours = 0
  a.each_index do |year|
    if !a[year].nil?
      a[year].each_index do |month|
        if !a[year][month].nil?
          hours = 0
          puts
          puts "#{month}/#{year}".center(line_width)
          weekdays.each { |v| print v.rjust(col_width) }
          puts
          m = a[year][month]
          m.fill(nil,m.size..31) 
          wday = 0
          m.each_index do |day|
            if DateTime.valid_civil?(year,month,day)
              wday = DateTime.civil(year,month,day).wday
              print "\r" + "\033[1C"*col_width*wday
              if !m[day].nil?
                print "#{m[day].to_s.rjust(col_width,' ').bold}"
                hours += a[year][month][day]
                week_hours += a[year][month][day]
              else 
                print "-".rjust(col_width)
              end
              if wday == 6
                puts week_hours.to_s.rjust(col_width)
                week_hours = 0
              end
            end
          end
          puts if wday != 6
          txt = "#{months[month]} #{year}: #{hours} hrs."
          txt += " / $#{format('%.2f',hours*$options[:rate].to_f)}" if $options[:rate]
          puts txt.center(line_width)
          all_hours += hours
        end
      end
    end
  end
  puts
  txt = "Total: #{$jobs.size} jobs, #{all_hours.to_s.bold} hrs."
  txt += " / $#{(format('%.2f',all_hours*$options[:rate].to_f)).bold}" if $options[:rate]
  puts txt
end

# accept reality
start_time = DateTime.now 
end_time = DateTime.now

# read console parameters
start_time = parsetime($options[:start_time]) if $options[:start_time]
end_time = parsetime($options[:end_time]) if $options[:end_time]
if $options[:duration]
  if !$options[:start] and  $options[:end]
    start_time = end_time - $options[:duration].to_f/24
    $options[:start] = true
  elsif !$options[:end] and $options[:start]
    end_time = start_time + $options[:duration].to_f/24
    $options[:end] = true
  elsif !$options[:start] and !$options[:end]
    puts "You gave a duration but no end or start time!".red
  else
    puts "You gave a duration but both end and start time!".red
  end
end

# check for duration parameter
end_time = start_time + $options[:duration].to_i/24  and 
end_time = start_time + $options[:duration].to_i/24 if $options[:time]

# load exisiting jobs from file
$jobs = []
if File.exist?($options[:filename])
  puts "Opening existing file '#{$options[:filename]}'" if $options[:verbose]
  File.open($options[:filename],"a+") do |f|
    f.readlines.each do |line| 
      $jobs << Job.from_s(line.chop)
    end
  end
  puts "read #{$jobs.size} jobs" if $options[:verbose]
end

# run commands
join $options[:join].collect{|c| c.to_i} if $options[:join]
drop $options[:drop].to_i if $options[:drop]
listjobs if $options[:list]
listjobs true if $options[:total]
if $options[:start] and $options[:end]
  startjob start_time, nil 
  endjob end_time, "Adding Job:" 
elsif $options[:start]
  startjob start_time
elsif $options[:end]
  endjob end_time 
elsif $options[:message]
  if !$jobs.empty? and !$jobs.last.finished?
    puts "Appending message to running job:" if $options[:verbose]
    puts $jobs.last
    $jobs.last.message += ($jobs.last.message.empty? ? "" : "\n") + enter_message(true,"Please enter a text to append to this message (empty line quits):") 
  else
    puts "No job running.".red
    print "Would you like to start a new one now (y/N)?"
    startjob DateTime.now if gets.strip.casecmp("y") == 0
  end
end
report if $options[:report]

# save jobs back into file
File.open($options[:filename],"w+") do |f|
  $jobs.each do |j|
    f.puts j.pack
  end
end




