use prettytable::{Attr, Cell, Row, Table};

pub struct TableOutput {
    table: Table,
}

impl TableOutput {
    pub fn new(header: Vec<&'static str>) -> TableOutput {
        let mut table = Table::new();
        let row_header = header
            .iter()
            .map(|item| Cell::new(item).with_style(Attr::Bold))
            .collect::<Vec<Cell>>();
        table.add_row(Row::new(row_header));
        TableOutput { table }
    }

    pub fn add_rows<'a>(&'a mut self, rows: Vec<Vec<String>>) -> &'a mut TableOutput {
        rows.into_iter().for_each(|item| {
            let cells = item
                .iter()
                .map(|cell| Cell::new(cell))
                .collect::<Vec<Cell>>();
            self.table.add_row(Row::new(cells));
        });

        self
    }

    pub fn printstd(&self) {
        self.table.printstd();
    }
}
