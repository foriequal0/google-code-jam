ROOT="/media/sf_C_DRIVE/Users/foriequal0/Downloads/codejam3/"

%.exe: %.cpp
	g++ -g -std=c++11 -o $@ $<
	cp $< $(ROOT)/$<

.SILENT:
run_%: %.exe
	./$< < $(ROOT)/$(PROB).in > $(ROOT)/$(PROB).out

.SILENT:
test_%: %.exe
	./$<

