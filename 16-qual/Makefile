%.exe: %.cpp
	g++ -std=c++11 -o $@ $<

.SILENT:
run_%: %.exe
	./$<
