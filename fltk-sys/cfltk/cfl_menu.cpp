#include "cfl_menu.h"
#include <Fl/Fl_Choice.H>
#include <Fl/Fl_Menu_Bar.H>
#include <Fl/Fl_Menu_Button.H>
#include <Fl/Fl_Menu_Item.H>

WIDGET_DEFINE(Fl_Menu_Bar)

WIDGET_DEFINE(Fl_Menu_Button)

WIDGET_DEFINE(Fl_Choice)

void Fl_Menu_Bar_add(Fl_Menu_Bar *self, const char *name, int shortcut,
                     Fl_Callback *cb, void *data, int flag) {
  self->add(name, shortcut, cb, data, flag);
}

Fl_Menu_Item *Fl_Menu_Bar_get_item(Fl_Menu_Bar *self, const char *name) {
  return (Fl_Menu_Item *)self->find_item(name);
}