

struct CalculadorNumerosTriangulares {
    ultimo_triangular : i128,
    ultimo_natural : i128
}

impl CalculadorNumerosTriangulares{
    fn new() -> CalculadorNumerosTriangulares{
        CalculadorNumerosTriangulares{ultimo_natural : 2 , ultimo_triangular : 1}
    }

    fn incrementa_triangular(&mut self){
        self.ultimo_triangular = self.ultimo_triangular + self.ultimo_natural;
        self.ultimo_natural = self.ultimo_natural + 1;
    }

    fn checa_triangular(&mut self, numero: i128) -> bool{
        if numero < self.ultimo_triangular {
            return false;
        }
        if numero == self.ultimo_triangular {
            self.incrementa_triangular();
            return true;
        }

        while self.ultimo_triangular < numero {
            self.incrementa_triangular();
            if numero == self.ultimo_triangular{
                return true;
            }
        }
        return false;
    }
}

struct CalculadorNumerosMagicos {
    tratador_numeros_triangulares : CalculadorNumerosTriangulares,
    impar_atual : i128,
    quadrado_atual : i128
}

impl CalculadorNumerosMagicos{
    fn new() -> CalculadorNumerosMagicos{
        CalculadorNumerosMagicos {tratador_numeros_triangulares : CalculadorNumerosTriangulares::new(), 
        impar_atual : 1, quadrado_atual : 1 }
    }

    fn calcula(&mut self){
        loop{
            let eh_magico : bool = self.tratador_numeros_triangulares.checa_triangular(self.quadrado_atual);
            if eh_magico {
                println!("{} é mágico", self.quadrado_atual);
            }
            self.impar_atual = self.impar_atual + 2;
            self.quadrado_atual = self.quadrado_atual + self.impar_atual;
        }
    }

}


fn main() {
    let mut calculador_magico = CalculadorNumerosMagicos::new();
    calculador_magico.calcula();

}
