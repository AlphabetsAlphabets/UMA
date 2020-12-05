class Link:
    def __init__(self, term):
        if " " in term:
            term = term.split(" ")
            self.term = "+".join(term)
        else:
            self.term = term
