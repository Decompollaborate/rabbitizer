PYI_TABLE_GEN    := ../tools/pyi_table_gen.sh

IINC            := -I ../include

TABLE_DIRS      := $(shell find . -type d)
TABLE_TEMPLATES := $(foreach dir,$(TABLE_DIRS),$(wildcard $(dir)/*.tablepyi.template))
TABLE_GENERATED := $(TABLE_TEMPLATES:%.tablepyi.template=%.pyi)

TABLE_DEP_FILES := $(TABLE_GENERATED:%.pyi=%.tablepyi.d)


all: tables

tables: $(TABLE_GENERATED)

clean:
	$(RM) -rf $(TABLE_GENERATED)

distclean: clean


.PHONY: all tables clean distclean
.DEFAULT_GOAL := all
.SECONDARY:


%.pyi: %.tablepyi.template
	cpp -P $(IINC) -M -MM -MMD -MP -MT $@ -MF $(<:.template=.d) $<
	$(PYI_TABLE_GEN) $< $@

-include $(TABLE_DEP_FILES)

# Print target for debugging
print-% : ; $(info $* is a $(flavor $*) variable set to [$($*)]) @true
