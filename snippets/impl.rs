impl User {
    fn created_at(&self) -> Tm {
        time::at(Timespec::new(self.created_at, 0))
    }

    fn updated_at(&self) -> Tm {
        time::at(Timespec::new(self.updated_at, 0))
    }
}
