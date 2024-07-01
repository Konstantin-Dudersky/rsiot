export def print_header [header: string] {
    print (ansi magenta_bold)
    print "================================================================================"
    print $"\n($header)\n"
    print "================================================================================"
    print (ansi reset)
}
