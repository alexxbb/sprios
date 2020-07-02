/*
    gcc -Wall big_surface1.c -o big_surface1 `pkg-config --cflags --libs gtk+-3.0`

    Tested on Ubuntu18.04 and GTK3.22
*/

#include<gtk/gtk.h>

static gdouble translate_x=0.0;
static gdouble translate_y=0.0;
static gdouble scale=1.0;

static cairo_surface_t* big_surface_new();
static void translate_x_spin_changed(GtkSpinButton *spin_button, gpointer data);
static void translate_y_spin_changed(GtkSpinButton *spin_button, gpointer data);
static void scale_spin_changed(GtkSpinButton *spin_button, gpointer data);
static gboolean da_drawing(GtkWidget *da, cairo_t *cr, cairo_surface_t *big_surface);

int main(int argc, char **argv)
{
    gtk_init(&argc, &argv);

    GtkWidget *window=gtk_window_new(GTK_WINDOW_TOPLEVEL);
    gtk_window_set_title(GTK_WINDOW(window), "Big Surface");
    gtk_window_set_default_size(GTK_WINDOW(window), 500, 500);
    gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
    g_signal_connect(window, "destroy", G_CALLBACK(gtk_main_quit), NULL);

    //Get a test surface.
    cairo_surface_t *big_surface=big_surface_new();

    GtkWidget *da=gtk_drawing_area_new();
    gtk_widget_set_hexpand(da, TRUE);
    gtk_widget_set_vexpand(da, TRUE);
    g_signal_connect(da, "draw", G_CALLBACK(da_drawing), big_surface);

    GtkWidget *scroll=gtk_scrolled_window_new(NULL, NULL);
    gtk_widget_set_hexpand(scroll, TRUE);
    gtk_widget_set_vexpand(scroll, TRUE);
    //Enougth drawing area size to scale the partition.
    gtk_widget_set_size_request(da, 1000, 1000);
    gtk_container_add(GTK_CONTAINER(scroll), da);

    GtkAdjustment *translate_x_adj=gtk_adjustment_new(0.0, 0.0, 5000.0, 50.0, 0.0, 0.0);
    GtkAdjustment *translate_y_adj=gtk_adjustment_new(0.0, 0.0, 5000.0, 50.0, 0.0, 0.0);
    GtkAdjustment *scale_adj=gtk_adjustment_new(1.0, 0.2, 2.0, 0.1, 0.0, 0.0);

    GtkWidget *translate_x_label=gtk_label_new("translate x");
    GtkWidget *translate_x_spin=gtk_spin_button_new(translate_x_adj, 50.0, 1);
    g_signal_connect(translate_x_spin, "value-changed", G_CALLBACK(translate_x_spin_changed), da);

    GtkWidget *translate_y_label=gtk_label_new("translate y");
    GtkWidget *translate_y_spin=gtk_spin_button_new(translate_y_adj, 50.0, 1);
    g_signal_connect(translate_y_spin, "value-changed", G_CALLBACK(translate_y_spin_changed), da);

    GtkWidget *scale_label=gtk_label_new("Scale");
    GtkWidget *scale_spin=gtk_spin_button_new(scale_adj, 0.2, 1);
    g_signal_connect(scale_spin, "value-changed", G_CALLBACK(scale_spin_changed), da);

    GtkWidget *grid=gtk_grid_new();
    gtk_grid_attach(GTK_GRID(grid), scroll, 0, 0, 3, 1);
    gtk_grid_attach(GTK_GRID(grid), translate_x_label, 0, 1, 1, 1);
    gtk_grid_attach(GTK_GRID(grid), translate_y_label, 1, 1, 1, 1);
    gtk_grid_attach(GTK_GRID(grid), scale_label, 2, 1, 1, 1);
    gtk_grid_attach(GTK_GRID(grid), translate_x_spin, 0, 2, 1, 1);
    gtk_grid_attach(GTK_GRID(grid), translate_y_spin, 1, 2, 1, 1);
    gtk_grid_attach(GTK_GRID(grid), scale_spin, 2, 2, 1, 1);

    gtk_container_add(GTK_CONTAINER(window), grid);

    gtk_widget_show_all(window);

    gtk_main();

    //Clean up.
    cairo_surface_destroy(big_surface);

    return 0;
}
static void translate_x_spin_changed(GtkSpinButton *spin_button, gpointer data)
{
    translate_x=gtk_spin_button_get_value(spin_button);
    gtk_widget_queue_draw(GTK_WIDGET(data));
}
static void translate_y_spin_changed(GtkSpinButton *spin_button, gpointer data)
{
    translate_y=gtk_spin_button_get_value(spin_button);
    gtk_widget_queue_draw(GTK_WIDGET(data));
}
static void scale_spin_changed(GtkSpinButton *spin_button, gpointer data)
{
    scale=gtk_spin_button_get_value(spin_button);
    gtk_widget_queue_draw(GTK_WIDGET(data));
}
static cairo_surface_t* big_surface_new()
{
    gint i=0;

    //Use gdk_cairo_surface_create_from_pixbuf() to read in a pixbuf. Try a test surface here.
    cairo_surface_t *big_surface=cairo_image_surface_create(CAIRO_FORMAT_ARGB32, 5000, 5000);
    cairo_t *cr=cairo_create(big_surface);

    //Paint the background.
    cairo_set_source_rgba(cr, 1.0, 1.0, 1.0, 1.0);
    cairo_paint(cr);

    //Draw some test grid lines.
    cairo_set_source_rgba(cr, 0.0, 1.0, 0.0, 1.0);
    for(i=0;i<50;i++)
    {
        cairo_move_to(cr, 0.0, (gdouble)i*100.0);
        cairo_line_to(cr, 5000.0, (gdouble)i*100.0);
        cairo_stroke(cr);
    }
    for(i=0;i<50;i++)
    {
        cairo_move_to(cr, (gdouble)i*100, 0.0);
        cairo_line_to(cr, (gdouble)i*100, 5000.0);
        cairo_stroke(cr);
    }

    cairo_set_source_rgba(cr, 0.0, 0.0, 1.0, 1.0);
    cairo_set_line_width(cr, 10.0);
    for(i=0;i<10;i++)
    {
        cairo_move_to(cr, 0.0, (gdouble)i*500.0);
        cairo_line_to(cr, 5000.0, (gdouble)i*500.0);
        cairo_stroke(cr);
    }
    for(i=0;i<10;i++)
    {
        cairo_move_to(cr, (gdouble)i*500.0, 0.0);
        cairo_line_to(cr, (gdouble)i*500.0, 5000.0);
        cairo_stroke(cr);
    }

    //Outside box.
    cairo_set_line_width(cr, 20.0);
    cairo_set_source_rgba(cr, 1.0, 0.0, 1.0, 1.0);
    cairo_rectangle(cr, 0.0, 0.0, 5000.0, 5000.0);
    cairo_stroke(cr);

    cairo_destroy(cr);

    return big_surface;
}
static gboolean da_drawing(GtkWidget *da, cairo_t *cr, cairo_surface_t *big_surface)
{
    gdouble origin_x=translate_x;
    gdouble origin_y=translate_y;

    //Some constraints.
    if(translate_x>4500.0) origin_x=4500.0;
    if(translate_y>4500.0) origin_y=4500.0;

    cairo_set_source_rgba(cr, 0.0, 0.0, 0.0, 1.0);
    cairo_paint(cr);

    //Partition the big surface.
    cairo_surface_t *little_surface=cairo_surface_create_for_rectangle(big_surface, origin_x, origin_y, 500.0, 500.0);

    cairo_scale(cr, scale, scale);
    cairo_set_source_surface(cr, little_surface, 0.0, 0.0);
    cairo_paint(cr);

    cairo_surface_destroy(little_surface);
    return FALSE;
}